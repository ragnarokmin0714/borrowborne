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
            let puzzle_id = app.current_puzzle().id.clone();
            let hints = app.current_puzzle().hints.clone();
            let toolbox = app.current_puzzle().toolbox.clone();
            let stained_here = app
                .progress
                .bloodstain
                .as_ref()
                .is_some_and(|s| s.puzzle_id == puzzle_id);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new(scene).size(14.0));

                // The corpse run: your echoes lie at this very door.
                if stained_here {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(format!("☠ {}", tr.stain_here))
                            .color(BLOOD)
                            .italics(),
                    );
                }

                // The toolbox: free, always shown — the syntax and
                // methods this puzzle may call for, named but not
                // explained. A nudge that competes with neither the
                // echo economy nor the paid hints below.
                if !toolbox.is_empty() {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(RichText::new(tr.toolbox_label).strong().size(13.0));
                    ui.horizontal_wrapped(|ui| {
                        for tool in &toolbox {
                            ui.label(RichText::new(tool).monospace().size(12.0).color(RUNE_GOLD));
                        }
                    });
                }

                if hints.is_empty() {
                    return;
                }
                ui.add_space(10.0);
                ui.separator();

                // Merciful lays the whole ladder open at once — a
                // beginner should never be stranded one paid tier short
                // of the near-solution. Nightfarer reveals one paid
                // whisper at a time.
                let merciful = app.difficulty == borrowborne_core::Difficulty::Easy;
                let revealed = if merciful {
                    hints.len()
                } else {
                    app.hints_shown
                };
                for hint in hints.iter().take(revealed) {
                    ui.label(
                        RichText::new(format!("🕯 {hint}"))
                            .italics()
                            .color(RUNE_GOLD),
                    );
                    ui.add_space(2.0);
                }
                if merciful {
                    // The lantern is already fully lit; nothing to buy.
                } else if app.hints_shown < hints.len() {
                    let cost =
                        borrowborne_core::Progress::hint_cost(app.hints_shown, app.difficulty);
                    let label = format!(
                        "{} — {cost}◉ ({}/{})",
                        tr.hint_whisper,
                        app.hints_shown + 1,
                        hints.len()
                    );
                    let affordable = app.progress.echoes >= cost;
                    if ui
                        .add_enabled(affordable, egui::Button::new(label))
                        .clicked()
                        && app.progress.buy_hint(app.hints_shown, app.difficulty)
                    {
                        app.hints_shown += 1;
                        app.dirty = true; // the purse changed: flush
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
    let (monster_id, monster_title) = {
        let p = app.current_puzzle();
        (p.id.clone(), p.title.clone())
    };
    let slain = app.progress.solved.contains(&monster_id);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(4.0);

        // The encounter: the trial is the monster, its health bar the
        // gate. Pure presentation — only a Passed verdict can drain it.
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("☠ {monster_title}"))
                    .strong()
                    .color(BLOOD),
            );
            let hp = ui.ctx().animate_value_with_time(
                egui::Id::new(("monster-hp", monster_id)),
                if slain { 0.0 } else { 1.0 },
                0.8,
            );
            let bar = ui.add(
                egui::ProgressBar::new(hp)
                    .desired_width(ui.available_width())
                    .fill(BLOOD),
            );
            app.encounter_bar = bar.rect;
        });

        ui.add_space(4.0);
        ui.label(RichText::new(tr.editor_hint).weak().italics());
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .id_source("editor")
            .max_height(ui.available_height() * 0.55)
            .show(ui, |ui| {
                // Live syntax highlighting: the editor lays its text
                // out through the highlighter instead of one flat
                // color. egui_extras caches the result per frame.
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut job =
                        egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, text, "rs");
                    // The highlighter lays out at its own (smaller) font;
                    // force every section to the editor's Monospace size
                    // so the text is as large as intended AND the caret
                    // — sized from the text style — matches the glyphs.
                    let mono = egui::TextStyle::Monospace.resolve(ui.style());
                    for section in &mut job.sections {
                        section.format.font_id = mono.clone();
                    }
                    job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(job))
                };
                ui.add(
                    egui::TextEdit::multiline(&mut app.code)
                        .code_editor()
                        .desired_rows(16)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        ui.add_space(2.0);
        ui.label(RichText::new(tr.editor_keys).weak().small());

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
            verdict_view::show(ui, tr, &verdict, &app.code);
        }
    });
}
