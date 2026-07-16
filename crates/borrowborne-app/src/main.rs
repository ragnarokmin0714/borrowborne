//! Borrowborne entry points: a native window, or a browser canvas via
//! trunk + wasm-bindgen (see `index.html`).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use borrowborne_app::app::BorrowborneApp;
#[cfg(not(target_arch = "wasm32"))]
use borrowborne_core::constants::APP_NAME;
#[cfg(not(target_arch = "wasm32"))]
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "borrowborne_canvas",
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(BorrowborneApp::new(cc)))),
            )
            .await
            .expect("failed to start Borrowborne in the browser");
    });
}
