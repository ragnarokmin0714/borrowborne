//! The curriculum: chapters (map regions) and puzzles (locked doors).
//!
//! Content lives in `content/chapters/*.ron` — adding a puzzle or a
//! whole region is a data edit, never a code change. [`loader`] parses
//! the files; [`concept`] names the skill-tree nodes a puzzle teaches.

mod concept;
mod loader;

pub use concept::Concept;
pub use loader::{load_dir, parse_chapter};

use serde::{Deserialize, Serialize};

/// The whole game: every chapter, in learning order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    pub chapters: Vec<Chapter>,
}

impl Curriculum {
    /// Find a puzzle anywhere in the curriculum by its id.
    pub fn puzzle(&self, id: &str) -> Option<&Puzzle> {
        self.chapters
            .iter()
            .flat_map(|c| &c.puzzles)
            .find(|p| p.id == id)
    }

    /// Total number of puzzles across all chapters.
    pub fn puzzle_count(&self) -> usize {
        self.chapters.iter().map(|c| c.puzzles.len()).sum()
    }
}

/// One map region = one chapter of Rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Stable id, e.g. `"ownership-forest"`. Progress keys refer to it.
    pub id: String,
    /// Display name of the region, e.g. `"Ownership Forest"`.
    pub name: String,
    /// Flavor line shown on the map.
    pub tagline: String,
    /// Hidden from the map unless the hunter walks the hardcore
    /// covenant — the algorithm dungeon is the Unforgiven's reward.
    /// Optional; defaults to a normal, always-visible region.
    #[serde(default)]
    pub hardcore_only: bool,
    /// Puzzles in intended solving order.
    pub puzzles: Vec<Puzzle>,
}

impl Chapter {
    /// The distinct concepts this region teaches, in first-seen order —
    /// the skill nodes shown for it in the journal.
    pub fn concepts(&self) -> Vec<Concept> {
        let mut seen = Vec::new();
        for concept in self.puzzles.iter().flat_map(|p| &p.concepts) {
            if !seen.contains(concept) {
                seen.push(*concept);
            }
        }
        seen
    }
}

/// One locked door: a scene, a starting spell, and hidden trials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    /// Stable id, unique across the whole curriculum.
    pub id: String,
    /// Short title shown above the scene.
    pub title: String,
    /// The NPC's statement of the problem (the visible puzzle text).
    pub scene: String,
    /// Rust concepts this puzzle teaches; nodes it lights up.
    pub concepts: Vec<Concept>,
    /// Code preloaded into the editor. Usually does not compile yet.
    pub starter_code: String,
    /// Hidden trial appended to the player's code by the harness.
    /// Plain Rust statements; `assert!` failures become
    /// [`crate::Verdict::TrialFailed`] via the harness marker.
    pub trial: String,
    /// Tiered hints, vaguest first: concept nudge → faulty line →
    /// near-solution. At most three; optional in content files.
    #[serde(default)]
    pub hints: Vec<String>,
    /// Free, always-shown nudges: the syntax, methods, or types this
    /// puzzle may call for (e.g. `.unwrap_or()`, `match`, `Vec<T>`).
    /// A step short of a hint — it names tools without saying how to
    /// use them — so it costs nothing and is not gated. Optional.
    #[serde(default)]
    pub toolbox: Vec<String>,
    /// Canonical solution. Never shown to the player — it exists so the
    /// content gate test can prove every puzzle is solvable (and every
    /// starter is not already a solution).
    #[serde(default)]
    pub solution: String,
}
