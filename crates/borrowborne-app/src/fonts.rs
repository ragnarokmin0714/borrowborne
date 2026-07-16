//! Font setup: a system CJK fallback so Traditional Chinese and
//! Japanese chrome renders. Follows gamegene's approach: read a system
//! font once, append it as the last fallback for both families.

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

/// Read a system CJK font once, if one is present.
pub fn load_cjk() -> Option<Vec<u8>> {
    const CANDIDATES: &[&str] = &[
        // Windows — always present on Windows 10/11.
        r"C:\Windows\Fonts\msjh.ttc",
        r"C:\Windows\Fonts\msjhl.ttc",
        r"C:\Windows\Fonts\msyh.ttc",
        // Linux / SteamOS — Noto CJK / WenQuanYi.
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        // macOS.
        "/System/Library/Fonts/PingFang.ttc",
    ];
    CANDIDATES.iter().find_map(|p| std::fs::read(p).ok())
}

/// Install the default fonts plus the CJK fallback (when found) for
/// both the proportional and monospace families.
pub fn apply(ctx: &egui::Context, cjk: &Option<Vec<u8>>) {
    let mut fonts = FontDefinitions::default();
    if let Some(bytes) = cjk {
        fonts
            .font_data
            .insert("cjk".to_owned(), FontData::from_owned(bytes.clone()));
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .push("cjk".to_owned());
        }
    }
    ctx.set_fonts(fonts);
}
