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

        // The covenant: Merciful frees the lantern, Nightfarer is the
        // balance, and the Unforgiven never saves and wipes on death.
        let name_of = |d: Difficulty| match d {
            Difficulty::Easy => tr.difficulty_easy,
            Difficulty::Normal => tr.difficulty_normal,
            Difficulty::Hardcore => tr.difficulty_hardcore,
        };
        ui.horizontal(|ui| {
            let indent = (ui.available_width() - 220.0).max(0.0) / 2.0;
            ui.add_space(indent);
            ui.label(RichText::new(tr.difficulty_label).weak());
            let mut diff = app.difficulty;
            egui::ComboBox::from_id_source("difficulty")
                .selected_text(name_of(diff))
                .show_ui(ui, |ui| {
                    for d in Difficulty::ALL {
                        ui.selectable_value(&mut diff, d, name_of(d));
                    }
                });
            if diff != app.difficulty {
                app.difficulty = diff;
                app.dirty = true;
            }
        })
        .response
        .on_hover_text(tr.difficulty_hint);

        // A standing system note so the difference is never a mystery:
        // spell out what the chosen covenant actually does, in place.
        let effect = match app.difficulty {
            Difficulty::Easy => tr.difficulty_effect_easy,
            Difficulty::Normal => tr.difficulty_effect_normal,
            Difficulty::Hardcore => tr.difficulty_effect_hardcore,
        };
        let effect_color = if app.difficulty == Difficulty::Hardcore {
            BLOOD
        } else {
            ui.visuals().weak_text_color()
        };
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(effect).small().italics().color(effect_color));
        });
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut enter: Option<usize> = None;
            let mut shown_any = false;

            for ix in 0..chapter_count {
                // Hardcore-only regions (the dungeon) stay off the map
                // unless the Unforgiven covenant is walked.
                if !app.region_visible(ix) {
                    continue;
                }
                let chapter = &app.curriculum.chapters[ix];
                let unlocked = app.region_enterable(ix);
                let hardcore = chapter.hardcore_only;
                let solved = app.progress.solved_in(chapter);
                let total = chapter.puzzles.len();
                let (name, tagline) = (chapter.name.clone(), chapter.tagline.clone());

                // The road between nodes (only after the first shown).
                if shown_any {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("│\n▼").weak().small());
                    });
                }
                shown_any = true;

                // The dungeon reads in blood; other open regions in gold.
                let border = if hardcore {
                    BLOOD
                } else if unlocked {
                    RUNE_GOLD
                } else {
                    ui.visuals().weak_text_color()
                };
                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style())
                        .stroke(egui::Stroke::new(1.0_f32, border))
                        .show(ui, |ui| {
                            ui.set_width(360.0);
                            ui.label(
                                RichText::new(if unlocked {
                                    name
                                } else {
                                    format!("🔒 {name}")
                                })
                                .strong()
                                .size(17.0)
                                .color(if hardcore {
                                    BLOOD
                                } else {
                                    ui.visuals().text_color()
                                }),
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
