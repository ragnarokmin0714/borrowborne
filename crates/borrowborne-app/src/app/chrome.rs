//! Top bar: identity, language, lives, progress.

use eframe::egui::{self, RichText};

use borrowborne_core::constants::APP_NAME;

use crate::i18n::Lang;
use crate::theme::{BLOOD, RUNE_GOLD};

use super::BorrowborneApp;

pub fn top_bar(app: &mut BorrowborneApp, ctx: &egui::Context) {
    let tr = app.lang.strings();
    egui::TopBottomPanel::top("chrome").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new(APP_NAME).strong().size(20.0).color(BLOOD));
            ui.label(RichText::new(tr.tagline).italics().weak());
            if ui.button(tr.map_button).clicked() {
                app.show_map();
            }
            let speaker = if app.muted() { "🔇" } else { "🔊" };
            if ui.button(speaker).clicked() {
                app.toggle_mute();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::ComboBox::from_id_source("lang")
                    .selected_text(app.lang.label())
                    .show_ui(ui, |ui| {
                        for lang in Lang::ALL {
                            ui.selectable_value(&mut app.lang, lang, lang.label());
                        }
                    });
                ui.label(RichText::new(tr.language).weak());

                ui.separator();

                // Lives as hearts; the run's pulse.
                let hearts = "♥".repeat(app.progress.lives_left() as usize);
                ui.label(RichText::new(hearts).color(BLOOD));
                ui.label(RichText::new(tr.lives).weak());

                ui.separator();

                // The night's curse; hover for its rules. (🌑: a new
                // run rises under a new moon — and unlike U+1F70F, the
                // glyph exists in egui's fonts on every platform.)
                if let Some(curse) = app.active_curse() {
                    ui.label(RichText::new(format!("🌑 {}", curse.name)).color(RUNE_GOLD))
                        .on_hover_text(&curse.blurb);
                    ui.label(RichText::new(tr.curse_label).weak());
                    ui.separator();
                }

                // The purse, and the stain if echoes lie somewhere.
                ui.label(
                    RichText::new(format!("◉ {}", app.progress.echoes))
                        .color(BLOOD)
                        .strong(),
                );
                ui.label(RichText::new(tr.echoes).weak());
                if let Some(stain) = &app.progress.bloodstain {
                    let title = app
                        .curriculum
                        .puzzle(&stain.puzzle_id)
                        .map_or(stain.puzzle_id.as_str(), |p| p.title.as_str());
                    ui.label(
                        RichText::new(format!("☠ {} — {} {}", stain.amount, tr.stain_away, title))
                            .color(RUNE_GOLD)
                            .small(),
                    );
                }

                ui.separator();

                // ✝ not U+2670: the Syriac cross has no glyph in any
                // font we ship, and rendered as a box on the web.
                ui.label(
                    RichText::new(format!("✝ {}", app.progress.total_deaths))
                        .color(RUNE_GOLD)
                        .weak(),
                );
                ui.label(RichText::new(tr.deaths_total).weak());

                ui.separator();

                let completion = app.progress.completion(&app.curriculum);
                ui.add(
                    egui::ProgressBar::new(completion)
                        .desired_width(140.0)
                        .text(format!("{:.0}%", completion * 100.0)),
                );
                ui.label(RichText::new(tr.progress).weak());
            });
        });
        ui.add_space(4.0);
    });
}
