//! Every non-ASCII character in the app sources must have a glyph in
//! at least one font we actually ship: egui's four defaults plus the
//! embedded CJK subset. A miss renders as an empty box in production —
//! exactly the bug reported in issue #1 — and nothing else catches it.
//!
//! Fails? Either rerun `assets/make_cjk_subset.py` (char exists in
//! Noto CJK TC) or swap the character for one egui's fonts carry.

use std::path::{Path, PathBuf};

use borrowborne_app::fonts::CJK_SUBSET;
use eframe::egui;

/// Non-ASCII chars from every `.rs` under `src/`.
fn source_chars() -> Vec<char> {
    fn walk(dir: &Path, out: &mut String) {
        for entry in std::fs::read_dir(dir).expect("readable src dir").flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push_str(&std::fs::read_to_string(&path).expect("readable source"));
            }
        }
    }
    let mut text = String::new();
    walk(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut text,
    );
    let mut chars: Vec<char> = text
        .chars()
        // Skip format controls (variation selectors, ZWJ): they are
        // instructions, not glyphs.
        .filter(|&c| c as u32 > 0x7F && c != '\u{FE0F}' && c != '\u{200D}')
        .collect();
    chars.sort_unstable();
    chars.dedup();
    chars
}

#[test]
fn fonts_cover_every_source_glyph() {
    // The union of everything we ship: egui's defaults + our subset.
    let defaults = egui::FontDefinitions::default();
    let mut faces: Vec<Vec<u8>> = defaults
        .font_data
        .values()
        .map(|d| d.font.to_vec())
        .collect();
    faces.push(CJK_SUBSET.to_vec());
    let faces: Vec<ttf_parser::Face> = faces
        .iter()
        .map(|bytes| ttf_parser::Face::parse(bytes, 0).expect("shipped font must parse"))
        .collect();

    let missing: Vec<char> = source_chars()
        .into_iter()
        .filter(|&c| !faces.iter().any(|f| f.glyph_index(c).is_some()))
        .collect();

    assert!(
        missing.is_empty(),
        "no shipped font has a glyph for: {missing:?} — rerun \
         assets/make_cjk_subset.py, or swap the character for a covered one"
    );
}
