//! The world map: one node per region, joined by a night road.
//! Sealed regions show what it takes to break the seal.

use eframe::egui::{self, RichText};

use borrowborne_core::Difficulty;

use crate::theme::{BLOOD, RUNE_GOLD};

use super::BorrowborneApp;

pub fn central(app: &mut BorrowborneApp, ctx: &egui::Context) {
    let tr = app.lang.strings();
    let chapter_count = app.curriculum.chapters.len();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(tr.map_title).strong().size(22.0).color(BLOOD));
        });
        ui.add_space(4.0);

        // The hunter's name: editable in place, sanitized when focus
        // leaves, persisted like any other progress. An empty name is
        // the nameless outlander — shown as a localized hint, never
        // written into the save.
        ui.horizontal(|ui| {
            let indent = (ui.available_width() - 220.0).max(0.0) / 2.0;
            ui.add_space(indent);
            ui.label(RichText::new(tr.hunter_label).weak());
            let resp = ui.add(
                egui::TextEdit::singleline(&mut app.progress.hunter_name)
                    .hint_text(tr.hunter_default)
                    .desired_width(160.0),
            );
            if resp.changed() {
                app.dirty = true;
            }
            if resp.lost_focus() {
                app.progress.sanitize_name();
                app.dirty = true;
            }
        });
        ui.add_space(4.0);

        // The covenant: Merciful makes the lantern (hints) free, for
        // players who find the borrow checker punishment enough.
        ui.horizontal(|ui| {
            let indent = (ui.available_width() - 220.0).max(0.0) / 2.0;
            ui.add_space(indent);
            ui.label(RichText::new(tr.difficulty_label).weak());
            let mut diff = app.progress.difficulty;
            egui::ComboBox::from_id_source("difficulty")
                .selected_text(match diff {
                    Difficulty::Normal => tr.difficulty_normal,
                    Difficulty::Easy => tr.difficulty_easy,
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut diff, Difficulty::Normal, tr.difficulty_normal);
                    ui.selectable_value(&mut diff, Difficulty::Easy, tr.difficulty_easy);
                });
            if diff != app.progress.difficulty {
                app.progress.difficulty = diff;
                app.dirty = true;
            }
        })
        .response
        .on_hover_text(tr.difficulty_hint);

        // A standing system note so the difference is never a mystery:
        // spell out what the chosen covenant actually does, in place.
        let effect = match app.progress.difficulty {
            Difficulty::Normal => tr.difficulty_effect_normal,
            Difficulty::Easy => tr.difficulty_effect_easy,
        };
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(effect).weak().small().italics());
        });
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut enter: Option<usize> = None;

            for ix in 0..chapter_count {
                let chapter = &app.curriculum.chapters[ix];
                let unlocked = app.progress.chapter_unlocked(&app.curriculum, ix);
                let solved = app.progress.solved_in(chapter);
                let total = chapter.puzzles.len();
                let (name, tagline) = (chapter.name.clone(), chapter.tagline.clone());

                // The road between nodes.
                if ix > 0 {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("│\n▼").weak().small());
                    });
                }

                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style())
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            if unlocked {
                                RUNE_GOLD
                            } else {
                                ui.visuals().weak_text_color()
                            },
                        ))
                        .show(ui, |ui| {
                            ui.set_width(360.0);
                            ui.label(
                                RichText::new(if unlocked {
                                    name
                                } else {
                                    format!("🔒 {name}")
                                })
                                .strong()
                                .size(17.0),
                            );
                            ui.label(RichText::new(tagline).italics().weak());
                            ui.add_space(4.0);
                            ui.label(format!("⚑ {solved} / {total}"));
                            ui.add_space(4.0);
                            if unlocked {
                                if ui
                                    .button(RichText::new(tr.map_enter).strong().color(BLOOD))
                                    .clicked()
                                {
                                    enter = Some(ix);
                                }
                            } else {
                                ui.label(
                                    RichText::new(tr.map_locked_hint).weak().small().italics(),
                                );
                            }
                        });
                });
            }

            if let Some(ix) = enter {
                app.enter_chapter(ix);
            }
        });
    });
}
