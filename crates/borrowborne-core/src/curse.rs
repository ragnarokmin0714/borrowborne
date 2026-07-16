//! Run curses: one random modifier per run, data-driven.
//!
//! Curses change the rules of a run, never the judge — a cursed cast
//! may be refused or taxed before it reaches the compiler, but code
//! that reaches the compiler is judged exactly as ever.

use serde::{Deserialize, Serialize};

/// What a curse does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurseEffect {
    /// Spells containing this snippet are refused before the judge is
    /// even summoned.
    ForbidSnippet(String),
    /// Every cast taxes this many echoes (floors at zero).
    EchoTax(u64),
}

/// One curse, as authored in `content/curses.ron`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curse {
    /// Stable id; the save refers to it.
    pub id: String,
    /// Display name, e.g. "Curse of the Twinless".
    pub name: String,
    /// One-line flavor + rules text shown on hover.
    pub blurb: String,
    pub effect: CurseEffect,
}

impl Curse {
    /// The refusal message when this curse rejects `code` outright,
    /// or `None` when the spell may proceed to the judge.
    pub fn refusal(&self, code: &str) -> Option<String> {
        match &self.effect {
            CurseEffect::ForbidSnippet(snippet) if code.contains(snippet.as_str()) => {
                Some(format!(
                    "{} — `{snippet}` is forbidden this run. The spell never leaves your lips.",
                    self.name
                ))
            }
            _ => None,
        }
    }

    /// Echoes taxed per cast under this curse.
    pub fn cast_tax(&self) -> u64 {
        match self.effect {
            CurseEffect::EchoTax(n) => n,
            _ => 0,
        }
    }
}

/// The full set of curses a run can roll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseBook {
    pub curses: Vec<Curse>,
}

impl CurseBook {
    /// Parse from RON text. `path` is only used for error text.
    pub fn parse(text: &str, path: &str) -> Result<Self, crate::CoreError> {
        ron::from_str(text).map_err(|source| crate::CoreError::ChapterParse {
            path: path.to_owned(),
            source,
        })
    }

    pub fn get(&self, id: &str) -> Option<&Curse> {
        self.curses.iter().find(|c| c.id == id)
    }

    /// Pick a curse for a fresh run from an arbitrary seed (the app
    /// feeds wall-clock entropy; determinism is not a goal here).
    pub fn roll(&self, seed: u64) -> Option<&Curse> {
        if self.curses.is_empty() {
            return None;
        }
        self.curses.get(seed as usize % self.curses.len())
    }
}
