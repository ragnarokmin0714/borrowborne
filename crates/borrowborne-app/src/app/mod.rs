//! The Borrowborne application: state, the frame loop, and casting.

mod chrome;
mod puzzle;
mod verdict_view;

use std::sync::mpsc;

use eframe::egui;

use borrowborne_core::constants::CHAPTERS_DIR;
use borrowborne_core::curriculum::{load_dir, parse_chapter};
use borrowborne_core::{Curriculum, Curse, CurseBook, Progress, Verdict};
#[cfg(not(target_arch = "wasm32"))]
use borrowborne_runner::{RustcLocal, Sandbox};

use crate::fx::Fx;
use crate::i18n::Lang;
use crate::{fonts, theme};

/// Compiled-in fallback so the game always has content — on the web
/// (no filesystem) this IS the content. Disk chapters win when present
/// (modding). Keep in learning order; new chapters must be added here
/// as well as on disk, or the web build will silently miss them.
const EMBEDDED_CHAPTERS: &[&str] = &[
    include_str!("../../../../content/chapters/01-newbie-village.ron"),
    include_str!("../../../../content/chapters/02-ownership-forest.ron"),
];

/// Curse book, embedded for the same reason as the chapters. Disk
/// wins when present.
const EMBEDDED_CURSES: &str = include_str!("../../../../content/curses.ron");

pub struct BorrowborneApp {
    curriculum: Curriculum,
    curse_book: CurseBook,
    progress: Progress,
    lang: Lang,

    chapter_ix: usize,
    puzzle_ix: usize,
    code: String,
    verdict: Option<Verdict>,
    /// Live while a cast is compiling/running on its worker thread.
    cast_rx: Option<mpsc::Receiver<Verdict>>,
    /// Center of the Cast button last frame; particle burst origin.
    cast_origin: egui::Pos2,
    /// The monster's health bar last frame; combat fx anchor here.
    encounter_bar: egui::Rect,
    /// Set when progress changed this frame: flush the save immediately
    /// instead of waiting for eframe's autosave tick, so a crash right
    /// after a pass (or a death) can never eat the verdict.
    dirty: bool,
    /// Hint tiers revealed for the current puzzle. Resets on puzzle
    /// change and on restart — hints re-seal themselves; only progress
    /// is forever.
    hints_shown: usize,

    fx: Fx,
}

impl BorrowborneApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::headless();
        if let Some(storage) = cc.storage {
            if let Some(p) = eframe::get_value(storage, "progress") {
                app.progress = p;
                // A corrupt save deserializes to None and we start
                // fresh; a *stale* one (old content, bad counters)
                // gets reconciled instead of poisoning the run.
                app.progress.rebuild(&app.curriculum);
            }
            if let Some(l) = eframe::get_value(storage, "lang") {
                app.lang = l;
            }
        }
        app.ensure_curse();
        theme::apply(&cc.egui_ctx);
        fonts::apply(&cc.egui_ctx, &fonts::load_cjk());
        app
    }

    /// State without a window or storage — the layout probe's entry.
    pub fn headless() -> Self {
        let curriculum = load_dir(std::path::Path::new(CHAPTERS_DIR)).unwrap_or_else(|_| {
            let chapters = EMBEDDED_CHAPTERS
                .iter()
                .map(|text| parse_chapter(text, "embedded").expect("embedded chapter must parse"))
                .collect();
            Curriculum { chapters }
        });
        let curse_book = std::fs::read_to_string("content/curses.ron")
            .ok()
            .and_then(|text| CurseBook::parse(&text, "content/curses.ron").ok())
            .unwrap_or_else(|| {
                CurseBook::parse(EMBEDDED_CURSES, "embedded").expect("embedded curses must parse")
            });
        let code = curriculum.chapters[0].puzzles[0].starter_code.clone();
        let mut app = Self {
            curriculum,
            curse_book,
            progress: Progress::default(),
            lang: Lang::default(),
            chapter_ix: 0,
            puzzle_ix: 0,
            code,
            verdict: None,
            cast_rx: None,
            cast_origin: egui::pos2(0.0, 0.0),
            encounter_bar: egui::Rect::NOTHING,
            dirty: false,
            hints_shown: 0,
            fx: Fx::default(),
        };
        app.ensure_curse();
        app
    }

    /// The active curse, if the save's id is still in the book.
    pub fn active_curse(&self) -> Option<&Curse> {
        self.progress
            .active_curse
            .as_deref()
            .and_then(|id| self.curse_book.get(id))
    }

    /// Roll a curse when none is active (fresh save, or content
    /// removed the one we had). Seeded from run statistics — cheap
    /// entropy that works on wasm too, where `SystemTime` panics.
    fn ensure_curse(&mut self) {
        if self.active_curse().is_some() {
            return;
        }
        self.reroll_curse();
    }

    /// A new night, a new moon: pick the next run's curse.
    fn reroll_curse(&mut self) {
        let seed = self.progress.total_deaths as u64 * 31
            + self.progress.solved.len() as u64 * 7
            + self.progress.echoes;
        self.progress.active_curse = self.curse_book.roll(seed).map(|c| c.id.clone());
    }

    pub fn current_puzzle(&self) -> &borrowborne_core::Puzzle {
        &self.curriculum.chapters[self.chapter_ix].puzzles[self.puzzle_ix]
    }

    pub fn casting(&self) -> bool {
        self.cast_rx.is_some()
    }

    /// Kick off a cast; the UI keeps breathing while the judge thinks.
    ///
    /// Native: a worker thread runs the local `rustc`. Web: the browser
    /// fetches the Rust Playground's execute API — same channel, same
    /// verdicts, different judge.
    fn cast(&mut self, ctx: &egui::Context) {
        if self.casting() {
            return;
        }
        // The curse strikes before the judge is even summoned: a
        // refusal never compiles; a tax is paid win or lose.
        if let Some(curse) = self.active_curse().cloned() {
            if let Some(refusal) = curse.refusal(&self.code) {
                let tr = self.lang.strings();
                self.fx.float_text(
                    self.encounter_bar.center_top(),
                    tr.combat_cursed,
                    theme::BLOOD,
                );
                self.verdict = Some(Verdict::CompileError(refusal));
                return;
            }
            let tax = curse.cast_tax();
            if tax > 0 {
                self.progress.echoes = self.progress.echoes.saturating_sub(tax);
                self.dirty = true;
            }
        }
        let (tx, rx) = mpsc::channel();
        self.cast_rx = Some(rx);
        self.verdict = None;
        let puzzle = self.current_puzzle().clone();
        let code = self.code.clone();
        let repaint = ctx.clone();

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let verdict = RustcLocal.evaluate(&puzzle, &code);
            let _ = tx.send(verdict);
            repaint.request_repaint();
        });

        #[cfg(target_arch = "wasm32")]
        {
            use borrowborne_runner::playground;
            let mut request = ehttp::Request::post(
                playground::EXECUTE_URL,
                playground::request_body(&puzzle, &code),
            );
            request
                .headers
                .insert("Content-Type".to_owned(), "application/json".to_owned());
            ehttp::fetch(request, move |result| {
                let verdict = match result {
                    Ok(resp) => playground::parse_response(resp.status, &resp.bytes),
                    Err(e) => Verdict::CompileError(format!("the far judge is unreachable: {e}")),
                };
                let _ = tx.send(verdict);
                repaint.request_repaint();
            });
        }
    }

    /// Collect a finished cast, apply the rules, fire the drama.
    fn poll_cast(&mut self) {
        let Some(rx) = &self.cast_rx else { return };
        let Ok(verdict) = rx.try_recv() else { return };
        self.cast_rx = None;

        let puzzle = self.current_puzzle();
        let (id, concepts) = (puzzle.id.clone(), puzzle.concepts.clone());
        let purse_before = self.progress.echoes;
        let run_ended = self.progress.record(&id, &concepts, &verdict);
        if run_ended {
            // The run is over; the next night rises under a new moon.
            self.reroll_curse();
        }
        self.dirty = true;

        // Combat theater: the verdict decides, these only perform it.
        let tr = self.lang.strings();
        let stage = self.encounter_bar.center_top();
        match &verdict {
            Verdict::Passed => {
                self.fx.on_kill(self.encounter_bar);
                self.fx.on_pass(self.cast_origin);
                let gained = self.progress.echoes.saturating_sub(purse_before);
                if gained > 0 {
                    self.fx
                        .float_text(stage, format!("+{gained} ◉"), theme::RUNE_GOLD);
                }
            }
            Verdict::CompileError(_) => {
                self.fx
                    .float_text(stage, tr.combat_miss, egui::Color32::GRAY);
            }
            Verdict::TrialFailed(_) => {
                self.fx.float_text(stage, tr.combat_blocked, theme::EMBER);
            }
            Verdict::Timeout => {
                self.fx
                    .float_text(stage, tr.combat_lost, egui::Color32::GRAY);
            }
            Verdict::Panicked(_) => {
                self.fx.on_death();
                let dropped = purse_before.saturating_sub(self.progress.echoes);
                if dropped > 0 {
                    self.fx
                        .float_text(stage, format!("☠ -{dropped} ◉"), theme::BLOOD);
                }
            }
        }
        self.verdict = Some(verdict);
    }

    /// Step to the neighboring puzzle, crossing chapter borders: the
    /// last door of one region leads into the first door of the next.
    fn goto_step(&mut self, forward: bool) {
        if self.casting() {
            return;
        }
        let count = self.curriculum.chapters[self.chapter_ix].puzzles.len();
        if forward {
            if self.puzzle_ix + 1 < count {
                self.puzzle_ix += 1;
            } else if self.chapter_ix + 1 < self.curriculum.chapters.len() {
                self.chapter_ix += 1;
                self.puzzle_ix = 0;
            } else {
                return;
            }
        } else if self.puzzle_ix > 0 {
            self.puzzle_ix -= 1;
        } else if self.chapter_ix > 0 {
            self.chapter_ix -= 1;
            self.puzzle_ix = self.curriculum.chapters[self.chapter_ix].puzzles.len() - 1;
        } else {
            return;
        }
        self.code = self.current_puzzle().starter_code.clone();
        self.verdict = None;
        self.hints_shown = 0;
    }

    /// Whether a step in the given direction leads anywhere.
    fn can_step(&self, forward: bool) -> bool {
        if forward {
            self.puzzle_ix + 1 < self.curriculum.chapters[self.chapter_ix].puzzles.len()
                || self.chapter_ix + 1 < self.curriculum.chapters.len()
        } else {
            self.puzzle_ix > 0 || self.chapter_ix > 0
        }
    }

    /// One full frame. Split from `update` so the headless layout
    /// probe can drive it without an eframe window.
    pub fn draw(&mut self, ctx: &egui::Context) {
        self.poll_cast();
        chrome::top_bar(self, ctx);
        puzzle::central(self, ctx);
        self.fx.tick(ctx);
    }
}

impl eframe::App for BorrowborneApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.draw(ctx);
        if self.dirty {
            if let Some(storage) = frame.storage_mut() {
                eframe::App::save(self, storage);
                storage.flush();
            }
            self.dirty = false;
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "progress", &self.progress);
        eframe::set_value(storage, "lang", &self.lang);
    }
}
