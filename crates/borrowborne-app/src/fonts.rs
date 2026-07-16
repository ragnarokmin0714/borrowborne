//! Font setup: CJK fallbacks so Traditional Chinese and Japanese
//! chrome renders everywhere.
//!
//! Two layers, appended after egui's defaults for both families:
//! 1. a system CJK font (full coverage, native only — the web has no
//!    filesystem),
//! 2. the embedded subset (~104 KiB) cut from Noto Sans CJK TC by
//!    `assets/make_cjk_subset.py`: every glyph the i18n strings use,
//!    plus the kana and fullwidth ranges. On the web it is the only
//!    CJK source; `fonts_cover_i18n` in the tests guards its coverage.

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

/// Subset of Noto Sans CJK TC (SIL OFL 1.1 — see
/// `assets/cjk-subset.LICENSE.txt`). Regenerate with
/// `assets/make_cjk_subset.py` after editing i18n strings.
pub const CJK_SUBSET: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/cjk-subset.otf"
));

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

/// Install the default fonts plus the CJK fallbacks (system font when
/// found, embedded subset always) for both families.
pub fn apply(ctx: &egui::Context, cjk: &Option<Vec<u8>>) {
    let mut fonts = FontDefinitions::default();
    if let Some(bytes) = cjk {
        fonts
            .font_data
            .insert("cjk".to_owned(), FontData::from_owned(bytes.clone()));
    }
    fonts
        .font_data
        .insert("cjk_subset".to_owned(), FontData::from_static(CJK_SUBSET));
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let list = fonts.families.entry(family).or_default();
        if cjk.is_some() {
            list.push("cjk".to_owned());
        }
        list.push("cjk_subset".to_owned());
    }
    ctx.set_fonts(fonts);
}
