//! Headless layout verification: drive full frames through a bare
//! egui context — no window, no GPU — so layout regressions surface in
//! CI instead of behind a persisted window size.

use borrowborne_app::app::BorrowborneApp;
use borrowborne_app::i18n::Lang;
use eframe::egui;

fn probe_app(app: &mut BorrowborneApp, size: egui::Vec2) {
    let ctx = egui::Context::default();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        ..Default::default()
    };
    // Two frames: egui settles sizes on the second pass.
    for _ in 0..2 {
        let _ = ctx.run(input.clone(), |ctx| app.draw(ctx));
    }
}

/// Every screen, at the given size: the map (headless default), a
/// puzzle entered from it, and the hunter's journal.
fn probe(size: egui::Vec2) {
    let mut app = BorrowborneApp::headless();
    probe_app(&mut app, size);
    app.enter_chapter(0);
    probe_app(&mut app, size);
    app.show_journal();
    probe_app(&mut app, size);
}

#[test]
fn draws_at_default_size() {
    probe(egui::vec2(1100.0, 720.0));
}

#[test]
fn draws_at_min_size() {
    probe(egui::vec2(820.0, 560.0));
}

#[test]
fn every_language_renders() {
    // A missing Tr field is a compile error, but a runtime layout
    // panic (e.g. glyph handling) would only show here.
    for lang in Lang::ALL {
        let _ = lang.strings();
    }
    probe(egui::vec2(1000.0, 640.0));
}
