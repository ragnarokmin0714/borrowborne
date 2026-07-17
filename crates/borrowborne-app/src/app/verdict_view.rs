//! The world's answer, performed: each verdict gets a voice and a
//! color, and raw compiler output stays legible in monospace.

use eframe::egui::{self, Color32, RichText};

use borrowborne_core::{Grade, Verdict};

use crate::i18n::Tr;
use crate::theme::{BLOOD, EMBER, RUNE_GOLD};

pub fn show(ui: &mut egui::Ui, tr: &Tr, verdict: &Verdict) {
    let (color, title, body, detail): (Color32, &str, &str, Option<&str>) = match verdict {
        Verdict::Passed { .. } => (RUNE_GOLD, tr.verdict_pass_title, tr.verdict_pass_body, None),
        Verdict::CompileError(diag) => (
            EMBER,
            tr.verdict_compile_title,
            tr.verdict_compile_body,
            Some(diag),
        ),
        Verdict::TrialFailed(msg) => (
            EMBER,
            tr.verdict_trial_title,
            tr.verdict_trial_body,
            Some(msg),
        ),
        Verdict::Panicked(msg) => (
            BLOOD,
            tr.verdict_death_title,
            tr.verdict_death_body,
            Some(msg),
        ),
        Verdict::Timeout => (
            EMBER,
            tr.verdict_timeout_title,
            tr.verdict_timeout_body,
            None,
        ),
    };

    egui::Frame::group(ui.style())
        .stroke(egui::Stroke::new(1.0_f32, color))
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong().size(17.0).color(color));
            ui.label(body);

            // Speed grade: how fast the trial itself ran.
            if let Verdict::Passed { trial_millis } = verdict {
                let grade = Grade::from_millis(*trial_millis);
                ui.label(
                    RichText::new(format!("⚡ {} — {trial_millis} ms", grade.letter()))
                        .strong()
                        .color(RUNE_GOLD),
                );
            }
            let Some(text) = detail else { return };

            // A known compiler error becomes an NPC performance; the
            // raw diagnostic stays available but collapsed. Unknown
            // errors show raw and open — no voice is better than a
            // wrong voice.
            let voice = matches!(verdict, Verdict::CompileError(_))
                .then(|| first_ecode(text))
                .flatten()
                .and_then(|code| tr.voice_for(code));
            if let Some(v) = voice {
                ui.label(RichText::new(v.line).italics().size(14.0).color(color));
                ui.label(v.note);
                egui::CollapsingHeader::new(tr.raw_diagnostic)
                    .id_source("verdict-raw")
                    .show(ui, |ui| diagnostic_text(ui, text));
            } else {
                diagnostic_text(ui, text);
            }
        });
}

fn diagnostic_text(ui: &mut egui::Ui, text: &str) {
    egui::ScrollArea::vertical()
        .id_source("verdict-detail")
        .max_height(180.0)
        .show(ui, |ui| {
            ui.label(RichText::new(text).monospace().size(12.0));
        });
}

/// The first `error[EXXXX]` code in a rustc diagnostic, if any.
fn first_ecode(diag: &str) -> Option<&str> {
    let start = diag.find("error[")? + "error[".len();
    let rest = &diag[start..];
    let end = rest.find(']')?;
    let code = &rest[..end];
    let well_formed =
        code.len() == 5 && code.starts_with('E') && code[1..].bytes().all(|b| b.is_ascii_digit());
    well_formed.then_some(code)
}

#[cfg(test)]
mod tests {
    use super::first_ecode;

    #[test]
    fn finds_the_first_code() {
        let diag = "warning: x\nerror[E0382]: borrow of moved value: `key`\nerror[E0308]: y";
        assert_eq!(first_ecode(diag), Some("E0382"));
    }

    #[test]
    fn ignores_malformed_and_absent_codes() {
        assert_eq!(first_ecode("error: expected one of `,`"), None);
        assert_eq!(first_ecode("error[weird]: ?"), None);
        assert_eq!(first_ecode(""), None);
    }
}
