//! The outcome of casting a spell (compiling and running player code).
//!
//! Produced by the runner crate, consumed by the app, but defined here
//! so both depend on pure data instead of on each other.

use serde::{Deserialize, Serialize};

/// What the world said about the player's code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Compiled, ran, and every hidden trial held. The gate opens.
    Passed,
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
        matches!(self, Verdict::Passed)
    }
}
