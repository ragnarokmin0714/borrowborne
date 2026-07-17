//! The outcome of casting a spell (compiling and running player code).
//!
//! Produced by the runner crate, consumed by the app, but defined here
//! so both depend on pure data instead of on each other.

use serde::{Deserialize, Serialize};

use crate::constants::{GRADE_A_MILLIS, GRADE_S_MILLIS};

/// What the world said about the player's code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Compiled, ran, and every hidden trial held. The gate opens.
    /// Carries how long the trial itself ran (the harness times it),
    /// which decides the [`Grade`].
    Passed { trial_millis: u64 },
    /// `rustc` refused. The payload is the diagnostic text, which the
    /// app performs as the voice of the world.
    CompileError(String),
    /// Compiled and ran, but a hidden trial assertion failed.
    TrialFailed(String),
    /// The program panicked at runtime. Roguelike rules: permadeath.
    Panicked(String),
    /// Compilation or execution exceeded its time budget.
    Timeout,
}

impl Verdict {
    /// Whether this verdict costs the player a life.
    pub fn is_lethal(&self) -> bool {
        matches!(self, Verdict::Panicked(_))
    }

    /// Whether the gate opens.
    pub fn is_pass(&self) -> bool {
        matches!(self, Verdict::Passed { .. })
    }
}

/// Speed grade for a pass — LeetCode-honest: measured wall time under
/// the trial's own workload, not fake Big-O detection. Ordered so a
/// *smaller* discriminant is a *better* grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Grade {
    S,
    A,
    B,
}

impl Grade {
    pub fn from_millis(ms: u64) -> Self {
        if ms <= GRADE_S_MILLIS {
            Grade::S
        } else if ms <= GRADE_A_MILLIS {
            Grade::A
        } else {
            Grade::B
        }
    }

    pub fn letter(self) -> &'static str {
        match self {
            Grade::S => "S",
            Grade::A => "A",
            Grade::B => "B",
        }
    }
}
