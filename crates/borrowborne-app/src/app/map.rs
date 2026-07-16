//! The world map: one node per region, joined by a night road.
//! Sealed regions show what it takes to break the seal.

use eframe::egui::{self, RichText};

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
