//! The world's answer, performed: each verdict gets a voice and a
//! color, and raw compiler output stays legible in monospace.

use eframe::egui::{self, Color32, RichText};

use borrowborne_core::Verdict;

use crate::i18n::Tr;
use crate::theme::{BLOOD, EMBER, RUNE_GOLD};

pub fn show(ui: &mut egui::Ui, tr: &Tr, verdict: &Verdict) {
    let (color, title, body, detail): (Color32, &str, &str, Option<&str>) = match verdict {
        Verdict::Passed => (RUNE_GOLD, tr.verdict_pass_title, tr.verdict_pass_body, None),
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
            if let Some(text) = detail {
                egui::ScrollArea::vertical()
                    .id_source("verdict-detail")
                    .max_height(180.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new(text).monospace().size(12.0));
                    });
            }
        });
}
