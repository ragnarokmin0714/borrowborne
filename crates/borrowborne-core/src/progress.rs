//! Save model: what the player has conquered, and what it cost.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::constants::{
    CHAPTER_UNLOCK_FRACTION, ECHOES_PER_SOLVE, ECHO_BONUS_A, ECHO_BONUS_S, HINT_COSTS,
    LEGACY_DEFAULT_HUNTER_NAME, LIVES_PER_RUN, MAX_HUNTER_NAME_LEN, STARTING_ECHOES,
};
use crate::curriculum::{Concept, Curriculum};
use crate::verdict::{Grade, Verdict};

/// Blood echoes lying where a hunter died — Bloodborne rules:
/// re-solve that puzzle to reclaim them; die again holding echoes and
/// the new stain replaces this one, the old echoes lost forever.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bloodstain {
    /// Puzzle where the hunter fell.
    pub puzzle_id: String,
    /// Echoes waiting to be reclaimed.
    pub amount: u64,
}

/// Persistent player progress. Serialized by the app via eframe's
/// persistence; core only defines the shape and the rules.
///
/// The save stays deliberately small: solved ids, death counters and
/// the echo purse. Anything derivable — like `learned` — is recomputed
/// by [`Progress::rebuild`] on load, which doubles as corruption
/// recovery (stale ids dropped, counters clamped).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Blood echoes held. Currency for hints (and, later, curses'
    /// mercy). Old saves without the field start with the default
    /// purse rather than empty-handed.
    #[serde(default = "starting_echoes")]
    pub echoes: u64,
    /// Echoes dropped at the puzzle where the hunter last died.
    #[serde(default)]
    pub bloodstain: Option<Bloodstain>,
    /// Id of the run's active curse. Rolled by the app at run start
    /// and rerolled when a run ends; validated against the curse book
    /// on load.
    #[serde(default)]
    pub active_curse: Option<String>,
    /// What the world calls this hunter. Player-editable; sanitized by
    /// [`Progress::rebuild`]. Empty means "still the nameless
    /// outlander" — the app shows a localized default in that case.
    #[serde(default)]
    pub hunter_name: String,
    /// Best speed grade per solved puzzle (S beats A beats B).
    #[serde(default)]
    pub grades: HashMap<String, Grade>,
}

fn starting_echoes() -> u64 {
    STARTING_ECHOES
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            solved: HashSet::new(),
            learned: HashSet::new(),
            deaths: 0,
            total_deaths: 0,
            echoes: STARTING_ECHOES,
            bloodstain: None,
            active_curse: None,
            hunter_name: String::new(),
            grades: HashMap::new(),
        }
    }
}

impl Progress {
    /// Reconcile a loaded save with the current curriculum. Call once
    /// after deserializing, before play:
    ///
    /// - drops solved ids the curriculum no longer knows (renamed or
    ///   removed content, or a corrupt entry),
    /// - recomputes `learned` from what remains (it is never saved),
    /// - clamps death counters a bad save could have inflated,
    /// - clears a bloodstain pointing at a puzzle that no longer exists
    ///   (the echoes return to the purse — content edits should never
    ///   steal from the player).
    pub fn rebuild(&mut self, curriculum: &Curriculum) {
        self.solved.retain(|id| curriculum.puzzle(id).is_some());
        self.grades.retain(|id, _| curriculum.puzzle(id).is_some());
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
        if let Some(stain) = &self.bloodstain {
            if curriculum.puzzle(&stain.puzzle_id).is_none() {
                self.echoes += stain.amount;
                self.bloodstain = None;
            }
        }
        self.sanitize_name();
    }

    /// Keep the hunter's name printable and bounded. Whitespace-only
    /// names collapse to empty — the nameless outlander, shown under a
    /// localized default by the app. Saves from before the outlander
    /// carried a literal "Good Hunter"; that too means "never named".
    pub fn sanitize_name(&mut self) {
        let trimmed = self.hunter_name.trim();
        self.hunter_name = if trimmed == LEGACY_DEFAULT_HUNTER_NAME {
            String::new()
        } else {
            trimmed.chars().take(MAX_HUNTER_NAME_LEN).collect()
        };
    }

    /// Record a verdict for a puzzle. Returns `true` when this death
    /// ended the run (deaths reached [`LIVES_PER_RUN`]).
    pub fn record(&mut self, puzzle_id: &str, concepts: &[Concept], verdict: &Verdict) -> bool {
        if let Verdict::Passed { trial_millis } = verdict {
            let grade = Grade::from_millis(*trial_millis);
            // First solve pays base + speed bonus; an already-open
            // gate pays nothing, but a better grade is still kept.
            if self.solved.insert(puzzle_id.to_owned()) {
                self.echoes += ECHOES_PER_SOLVE
                    + match grade {
                        Grade::S => ECHO_BONUS_S,
                        Grade::A => ECHO_BONUS_A,
                        Grade::B => 0,
                    };
            }
            self.grades
                .entry(puzzle_id.to_owned())
                .and_modify(|best| *best = (*best).min(grade))
                .or_insert(grade);
            self.learned.extend(concepts.iter().copied());
            // The corpse run: passing the puzzle where you fell
            // reclaims what you dropped there.
            if let Some(stain) = &self.bloodstain {
                if stain.puzzle_id == puzzle_id {
                    self.echoes += stain.amount;
                    self.bloodstain = None;
                }
            }
            return false;
        }
        if verdict.is_lethal() {
            self.deaths += 1;
            self.total_deaths += 1;
            // Drop everything you hold where you fell. A new stain
            // replaces the old one — those echoes are gone for good.
            // Dying empty-handed leaves no stain and spares the old.
            if self.echoes > 0 {
                self.bloodstain = Some(Bloodstain {
                    puzzle_id: puzzle_id.to_owned(),
                    amount: self.echoes,
                });
                self.echoes = 0;
            }
            if self.deaths >= LIVES_PER_RUN {
                // Roguelike reset: the run ends, solved gates stay open
                // (knowledge survives death — that is the point).
                self.deaths = 0;
                return true;
            }
        }
        false
    }

    /// Price of the given hint tier (0-based).
    pub fn hint_cost(tier: usize) -> u64 {
        HINT_COSTS.get(tier).copied().unwrap_or(0)
    }

    /// Buy the given hint tier. Returns `false` (and deducts nothing)
    /// when the purse cannot cover it.
    pub fn buy_hint(&mut self, tier: usize) -> bool {
        let cost = Self::hint_cost(tier);
        if self.echoes < cost {
            return false;
        }
        self.echoes -= cost;
        true
    }

    /// Puzzles solved within one chapter.
    pub fn solved_in(&self, chapter: &crate::Chapter) -> usize {
        chapter
            .puzzles
            .iter()
            .filter(|p| self.solved.contains(&p.id))
            .count()
    }

    /// Whether a region's seal is broken. The first region is always
    /// open; each later one needs [`CHAPTER_UNLOCK_FRACTION`] of the
    /// region before it solved.
    pub fn chapter_unlocked(&self, curriculum: &Curriculum, ix: usize) -> bool {
        if ix == 0 {
            return true;
        }
        let Some(prev) = curriculum.chapters.get(ix - 1) else {
            return false;
        };
        if prev.puzzles.is_empty() {
            return true;
        }
        self.solved_in(prev) as f32 / prev.puzzles.len() as f32 >= CHAPTER_UNLOCK_FRACTION
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
