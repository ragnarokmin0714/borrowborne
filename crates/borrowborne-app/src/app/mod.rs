//! The Borrowborne application: state, the frame loop, and casting.

mod chrome;
mod puzzle;
mod verdict_view;

use std::sync::mpsc;

use eframe::egui;

use borrowborne_core::constants::CHAPTERS_DIR;
use borrowborne_core::curriculum::{load_dir, parse_chapter};
use borrowborne_core::{Curriculum, Progress, Verdict};
#[cfg(not(target_arch = "wasm32"))]
use borrowborne_runner::{RustcLocal, Sandbox};

use crate::fx::Fx;
use crate::i18n::Lang;
use crate::{fonts, theme};

/// Compiled-in fallback so the game always has content, wherever the
/// working directory is. Disk content (modding) wins when present.
const EMBEDDED_CHAPTER: &str = include_str!("../../../../content/chapters/02-ownership-forest.ron");

pub struct BorrowborneApp {
    curriculum: Curriculum,
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

    fx: Fx,
}

impl BorrowborneApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::headless();
        if let Some(storage) = cc.storage {
            if let Some(p) = eframe::get_value(storage, "progress") {
                app.progress = p;
            }
            if let Some(l) = eframe::get_value(storage, "lang") {
                app.lang = l;
            }
        }
        theme::apply(&cc.egui_ctx);
        fonts::apply(&cc.egui_ctx, &fonts::load_cjk());
        app
    }

    /// State without a window or storage — the layout probe's entry.
    pub fn headless() -> Self {
        let curriculum = load_dir(std::path::Path::new(CHAPTERS_DIR)).unwrap_or_else(|_| {
            let chapter =
                parse_chapter(EMBEDDED_CHAPTER, "embedded").expect("embedded chapter must parse");
            Curriculum {
                chapters: vec![chapter],
            }
        });
        let code = curriculum.chapters[0].puzzles[0].starter_code.clone();
        Self {
            curriculum,
            progress: Progress::default(),
            lang: Lang::default(),
            chapter_ix: 0,
            puzzle_ix: 0,
            code,
            verdict: None,
            cast_rx: None,
            cast_origin: egui::pos2(0.0, 0.0),
            fx: Fx::default(),
        }
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
        self.progress.record(&id, &concepts, &verdict);

        if verdict.is_pass() {
            self.fx.on_pass(self.cast_origin);
        } else if verdict.is_lethal() {
            self.fx.on_death();
        }
        self.verdict = Some(verdict);
    }

    /// Move to another puzzle in the current chapter.
    fn goto_puzzle(&mut self, ix: usize) {
        let count = self.curriculum.chapters[self.chapter_ix].puzzles.len();
        if ix >= count || self.casting() {
            return;
        }
        self.puzzle_ix = ix;
        self.code = self.current_puzzle().starter_code.clone();
        self.verdict = None;
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "progress", &self.progress);
        eframe::set_value(storage, "lang", &self.lang);
    }
}
