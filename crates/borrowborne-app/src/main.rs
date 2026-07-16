//! Borrowborne desktop entry point.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use borrowborne_app::app::BorrowborneApp;
use borrowborne_core::constants::APP_NAME;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([820.0, 560.0])
            .with_title(format!("{APP_NAME} v{}", env!("CARGO_PKG_VERSION"))),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(BorrowborneApp::new(cc)))),
    )
}
