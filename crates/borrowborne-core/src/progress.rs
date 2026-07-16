//! Save model: what the player has conquered, and what it cost.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::constants::LIVES_PER_RUN;
use crate::curriculum::{Concept, Curriculum};
use crate::verdict::Verdict;

/// Persistent player progress. Serialized by the app via eframe's
/// persistence; core only defines the shape and the rules.
///
/// The save stays deliberately small: only solved ids and death
/// counters go to disk. Anything derivable — like `learned` — is
/// recomputed by [`Progress::rebuild`] on load, which doubles as
/// corruption recovery (stale ids dropped, counters clamped).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Progress {
    /// Ids of solved puzzles. The single source of truth on disk.
    pub solved: HashSet<String>,
    /// Concepts learned (union of solved puzzles' concepts). Derived —
    /// never serialized; [`Progress::rebuild`] restores it on load.
    #[serde(skip)]
    pub learned: HashSet<Concept>,
    /// Deaths in the current run.
    pub deaths: u32,
    /// Total deaths, ever. The tombstone counter.
    pub total_deaths: u32,
}

impl Progress {
    /// Reconcile a loaded save with the current curriculum. Call once
    /// after deserializing, before play:
    ///
    /// - drops solved ids the curriculum no longer knows (renamed or
    ///   removed content, or a corrupt entry),
    /// - recomputes `learned` from what remains (it is never saved),
    /// - clamps death counters a bad save could have inflated.
    pub fn rebuild(&mut self, curriculum: &Curriculum) {
        self.solved.retain(|id| curriculum.puzzle(id).is_some());
        self.learned = self
            .solved
            .iter()
            .filter_map(|id| curriculum.puzzle(id))
            .flat_map(|p| p.concepts.iter().copied())
            .collect();
        // A run's deaths are always below the reset threshold; anything
        // else is save damage, not history.
        self.deaths = self.deaths.min(LIVES_PER_RUN.saturating_sub(1));
        self.total_deaths = self.total_deaths.max(self.deaths);
    }

    /// Record a verdict for a puzzle. Returns `true` when this death
    /// ended the run (deaths reached [`LIVES_PER_RUN`]).
    pub fn record(&mut self, puzzle_id: &str, concepts: &[Concept], verdict: &Verdict) -> bool {
        if verdict.is_pass() {
            self.solved.insert(puzzle_id.to_owned());
            self.learned.extend(concepts.iter().copied());
            return false;
        }
        if verdict.is_lethal() {
            self.deaths += 1;
            self.total_deaths += 1;
            if self.deaths >= LIVES_PER_RUN {
                // Roguelike reset: the run ends, solved gates stay open
                // (knowledge survives death — that is the point).
                self.deaths = 0;
                return true;
            }
        }
        false
    }

    /// Lives remaining in the current run.
    pub fn lives_left(&self) -> u32 {
        LIVES_PER_RUN.saturating_sub(self.deaths)
    }

    /// Fraction of the curriculum solved, for the progress bar.
    pub fn completion(&self, curriculum: &Curriculum) -> f32 {
        let total = curriculum.puzzle_count();
        if total == 0 {
            return 0.0;
        }
        self.solved.len() as f32 / total as f32
    }
}
