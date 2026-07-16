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
}
