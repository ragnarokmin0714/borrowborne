//! The embedded CJK subset must cover every CJK character the i18n
//! strings use — on the web it is the only CJK source, and a missing
//! glyph renders as an empty box no other test would catch.
//!
//! Fails? Rerun `assets/make_cjk_subset.py` (see its docstring).

use borrowborne_app::fonts::CJK_SUBSET;
use borrowborne_app::i18n::Lang;

/// Characters egui's default fonts cannot draw: CJK ideographs, kana,
/// CJK punctuation, fullwidth forms.
fn needs_cjk(c: char) -> bool {
    ('\u{3000}'..='\u{9FFF}').contains(&c) || ('\u{FF00}'..='\u{FFEF}').contains(&c)
}

#[test]
fn fonts_cover_i18n() {
    let face = ttf_parser::Face::parse(CJK_SUBSET, 0).expect("subset font must parse");

    let mut missing = Vec::new();
    for lang in Lang::ALL {
        let tr = lang.strings();
        // Every user-facing string in the Tr struct, plus the picker label.
        let mut text = String::from(lang.label());
        text.push_str(&all_strings(tr));
        for c in text.chars().filter(|&c| needs_cjk(c)) {
            if face.glyph_index(c).is_none() {
                missing.push(c);
            }
        }
    }
    missing.sort_unstable();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "subset font lacks glyphs for: {missing:?} — rerun assets/make_cjk_subset.py"
    );
}

/// Concatenate every field of Tr. Kept dumb on purpose: when a field
/// is added, the compiler does not force an update here, but the
/// glyphs of a forgotten field were almost certainly already used
/// elsewhere; the subset ranges (kana, punctuation) absorb the rest.
fn all_strings(tr: &borrowborne_app::i18n::Tr) -> String {
    let voices = [
        &tr.e0382, &tr.e0384, &tr.e0308, &tr.e0369, &tr.e0499, &tr.e0502, &tr.e0106, &tr.e0425,
    ];
    let mut s = [
        tr.language,
        tr.lives,
        tr.progress,
        tr.deaths_total,
        tr.cast,
        tr.casting,
        tr.reset_code,
        tr.next_puzzle,
        tr.prev_puzzle,
        tr.editor_hint,
        tr.solved_badge,
        tr.verdict_pass_title,
        tr.verdict_pass_body,
        tr.verdict_compile_title,
        tr.verdict_compile_body,
        tr.verdict_trial_title,
        tr.verdict_trial_body,
        tr.verdict_death_title,
        tr.verdict_death_body,
        tr.verdict_timeout_title,
        tr.verdict_timeout_body,
        tr.hint_whisper,
        tr.hint_exhausted,
        tr.echoes,
        tr.stain_here,
        tr.stain_away,
        tr.raw_diagnostic,
        tr.combat_miss,
        tr.combat_blocked,
        tr.combat_lost,
        tr.combat_cursed,
        tr.curse_label,
        tr.map_title,
        tr.map_button,
        tr.map_enter,
        tr.map_locked_hint,
    ]
    .concat();
    for v in voices {
        s.push_str(v.line);
        s.push_str(v.note);
    }
    s
}
