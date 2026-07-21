//! The hunter's journal: what you've endured, and what Rust you've
//! learned. The skill tree is the whole point of the game made visible
//! — every concept you light up is a piece of Rust you now own.

use eframe::egui::{self, RichText};

use crate::theme::{BLOOD, RUNE_GOLD};

use super::BorrowborneApp;

pub fn central(app: &mut BorrowborneApp, ctx: &egui::Context) {
    let tr = app.lang.strings();

    // Only count regions the hunter can actually see — the hardcore
    // dungeon stays a secret off the covenant, here as on the map.
    let visible: Vec<usize> = (0..app.curriculum.chapters.len())
        .filter(|&ix| app.region_visible(ix))
        .collect();

    let total_puzzles: usize = visible
        .iter()
        .map(|&ix| app.curriculum.chapters[ix].puzzles.len())
        .sum();
    let solved_puzzles: usize = visible
        .iter()
        .map(|&ix| app.progress.solved_in(&app.curriculum.chapters[ix]))
        .sum();

    // Every concept across the visible regions, and how many are lit.
    let mut all_concepts = Vec::new();
    for &ix in &visible {
        for c in app.curriculum.chapters[ix].concepts() {
            if !all_concepts.contains(&c) {
                all_concepts.push(c);
            }
        }
    }
    let mastered = all_concepts
        .iter()
        .filter(|c| app.progress.learned.contains(c))
        .count();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(tr.journal_title)
                    .strong()
                    .size(22.0)
                    .color(BLOOD),
            );
            let name = if app.progress.hunter_name.is_empty() {
                tr.hunter_default
            } else {
                app.progress.hunter_name.as_str()
            };
            ui.label(RichText::new(name).italics().color(RUNE_GOLD).size(16.0));
        });
        ui.add_space(10.0);

        // A compact stat line: trials, concepts, deaths, echoes.
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(format!(
                "{}  {solved_puzzles} / {total_puzzles}      {}  {mastered} / {}      ✝ {}      ◉ {}",
                tr.journal_trials,
                tr.journal_concepts,
                all_concepts.len(),
                app.progress.total_deaths,
                app.progress.echoes,
            )));
        });
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for &ix in &visible {
                let chapter = &app.curriculum.chapters[ix];
                let concepts = chapter.concepts();
                let done = concepts
                    .iter()
                    .filter(|c| app.progress.learned.contains(c))
                    .count();

                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(420.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&chapter.name).strong().size(15.0));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(format!("{done} / {}", concepts.len()))
                                            .weak()
                                            .small(),
                                    );
                                },
                            );
                        });
                        ui.add_space(2.0);
                        // The skill nodes: lit gold when learned, faint
                        // when still ahead of the hunter.
                        ui.horizontal_wrapped(|ui| {
                            for c in &concepts {
                                let lit = app.progress.learned.contains(c);
                                let text = RichText::new(c.label()).monospace().size(13.0);
                                let text = if lit {
                                    text.color(RUNE_GOLD).strong()
                                } else {
                                    text.color(ui.visuals().weak_text_color())
                                };
                                ui.label(text);
                                ui.label(RichText::new("·").weak());
                            }
                        });
                    });
                });
                ui.add_space(6.0);
            }
        });
    });
}
