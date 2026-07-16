//! The puzzle screen: scene on the left, spell editor and verdict in
//! the center.

use eframe::egui::{self, RichText};

use crate::theme::{BLOOD, RUNE_GOLD};

use super::{verdict_view, BorrowborneApp};

pub fn central(app: &mut BorrowborneApp, ctx: &egui::Context) {
    scene_panel(app, ctx);
    editor_panel(app, ctx);
}

fn scene_panel(app: &mut BorrowborneApp, ctx: &egui::Context) {
    let tr = app.lang.strings();
    let chapter = &app.curriculum.chapters[app.chapter_ix];
    let puzzle = app.current_puzzle();
    let solved = app.progress.solved.contains(&puzzle.id);
    let (chapter_name, tagline) = (chapter.name.clone(), chapter.tagline.clone());
    let (title, scene) = (puzzle.title.clone(), puzzle.scene.clone());

    egui::SidePanel::left("scene")
        .resizable(true)
        .default_width(340.0)
        .min_width(240.0)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            ui.label(RichText::new(chapter_name).strong().size(15.0));
            ui.label(RichText::new(tagline).italics().weak());
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(title).strong().size(18.0));
                if solved {
                    ui.label(RichText::new(tr.solved_badge).small().color(RUNE_GOLD));
                }
            });
            ui.add_space(6.0);
            let hints = app.current_puzzle().hints.clone();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new(scene).size(14.0));

                if hints.is_empty() {
                    return;
                }
                ui.add_space(10.0);
                ui.separator();
                // Revealed tiers stay visible; the lantern offers the
                // next one until it runs out of things to say.
                for hint in hints.iter().take(app.hints_shown) {
                    ui.label(
                        RichText::new(format!("🕯 {hint}"))
                            .italics()
                            .color(RUNE_GOLD),
                    );
                    ui.add_space(2.0);
                }
                if app.hints_shown < hints.len() {
                    let label = format!(
                        "{} ({}/{})",
                        tr.hint_whisper,
                        app.hints_shown + 1,
                        hints.len()
                    );
                    if ui.button(label).clicked() {
                        app.hints_shown += 1;
                    }
                } else {
                    ui.label(RichText::new(tr.hint_exhausted).weak().small());
                }
            });
        });
}

fn editor_panel(app: &mut BorrowborneApp, ctx: &egui::Context) {
    let tr = app.lang.strings();
    let casting = app.casting();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(4.0);
        ui.label(RichText::new(tr.editor_hint).weak().italics());
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .id_source("editor")
            .max_height(ui.available_height() * 0.55)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut app.code)
                        .code_editor()
                        .desired_rows(16)
                        .desired_width(f32::INFINITY),
                );
            });

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            let cast_label = if casting { tr.casting } else { tr.cast };
            let cast_btn = ui.add_enabled(
                !casting,
                egui::Button::new(RichText::new(cast_label).strong().size(15.0))
                    .fill(BLOOD.linear_multiply(0.8)),
            );
            app.cast_origin = cast_btn.rect.center();
            if cast_btn.clicked() {
                app.cast(ctx);
            }
            if casting {
                ui.spinner();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let next = ui
                    .add_enabled(
                        app.can_step(true) && !casting,
                        egui::Button::new(tr.next_puzzle),
                    )
                    .clicked();
                let prev = ui
                    .add_enabled(
                        app.can_step(false) && !casting,
                        egui::Button::new(tr.prev_puzzle),
                    )
                    .clicked();
                if ui
                    .add_enabled(!casting, egui::Button::new(tr.reset_code))
                    .clicked()
                {
                    app.code = app.current_puzzle().starter_code.clone();
                    app.verdict = None;
                }
                if next {
                    app.goto_step(true);
                }
                if prev {
                    app.goto_step(false);
                }
            });
        });

        ui.add_space(6.0);
        if let Some(verdict) = app.verdict.clone() {
            verdict_view::show(ui, tr, &verdict);
        }
    });
}
