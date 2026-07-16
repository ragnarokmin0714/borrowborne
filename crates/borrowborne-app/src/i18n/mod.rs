//! UI language strings.
//!
//! Each language is one file with a `static` [`Tr`] — the struct
//! guarantees at compile time that every language defines every
//! string. To add a language: a new file + a [`Lang`] variant, and the
//! compiler walks you through the rest.
//!
//! Only chrome is translated; puzzle *content* (scenes, code) stays in
//! its authored language — content localization is a ROADMAP item.

use serde::{Deserialize, Serialize};

mod en;
mod ja;
mod zh_hant;

/// Selectable UI language.
#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Lang {
    #[default]
    En,
    ZhHant,
    Ja,
}

impl Lang {
    pub fn strings(self) -> &'static Tr {
        match self {
            Lang::En => &en::EN,
            Lang::ZhHant => &zh_hant::ZH,
            Lang::Ja => &ja::JA,
        }
    }

    /// Native-script label for the language picker.
    pub fn label(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::ZhHant => "繁體中文",
            Lang::Ja => "日本語",
        }
    }

    pub const ALL: [Lang; 3] = [Lang::En, Lang::ZhHant, Lang::Ja];
}

/// One compiler error code, performed: an in-world line an NPC speaks,
/// plus a plain-language note about what actually went wrong.
pub struct Voice {
    pub line: &'static str,
    pub note: &'static str,
}

/// All user-facing UI strings for one language.
pub struct Tr {
    // Chrome.
    pub language: &'static str,
    pub lives: &'static str,
    pub progress: &'static str,
    pub deaths_total: &'static str,

    // Puzzle screen.
    pub cast: &'static str,
    pub casting: &'static str,
    pub reset_code: &'static str,
    pub next_puzzle: &'static str,
    pub prev_puzzle: &'static str,
    pub editor_hint: &'static str,
    pub solved_badge: &'static str,

    // Verdicts, performed as the voice of the world.
    pub verdict_pass_title: &'static str,
    pub verdict_pass_body: &'static str,
    pub verdict_compile_title: &'static str,
    pub verdict_compile_body: &'static str,
    pub verdict_trial_title: &'static str,
    pub verdict_trial_body: &'static str,
    pub verdict_death_title: &'static str,
    pub verdict_death_body: &'static str,
    pub verdict_timeout_title: &'static str,
    pub verdict_timeout_body: &'static str,

    // Hints.
    pub hint_whisper: &'static str,
    pub hint_exhausted: &'static str,

    // Blood echoes.
    pub echoes: &'static str,
    pub stain_here: &'static str,
    pub stain_away: &'static str,

    // Raw compiler output disclosure.
    pub raw_diagnostic: &'static str,

    // The most common compiler errors, performed as NPC dialogue.
    pub e0382: Voice, // use of moved value
    pub e0384: Voice, // assign twice to immutable
    pub e0308: Voice, // mismatched types
    pub e0369: Voice, // operator not supported between types
    pub e0499: Voice, // two mutable borrows
    pub e0502: Voice, // mutable + shared borrow clash
    pub e0106: Voice, // missing lifetime
    pub e0425: Voice, // unresolved name
}

impl Tr {
    /// The in-world voice for a compiler error code, if we know it.
    pub fn voice_for(&self, code: &str) -> Option<&Voice> {
        Some(match code {
            "E0382" => &self.e0382,
            "E0384" => &self.e0384,
            "E0308" => &self.e0308,
            "E0369" => &self.e0369,
            "E0499" => &self.e0499,
            "E0502" => &self.e0502,
            "E0106" => &self.e0106,
            "E0425" => &self.e0425,
            _ => return None,
        })
    }
}
