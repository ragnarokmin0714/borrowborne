//! Wraps player code and the hidden trial into one compilation unit.

use borrowborne_core::Puzzle;

/// Printed on success so the runner can tell a clean pass from a
/// program that merely exited 0 without running the trial.
pub const PASS_MARKER: &str = "__BORROWBORNE_PASS__";

/// Trial assertion messages start with this; a panic carrying it is a
/// failed trial, any other panic is the player's own (permadeath).
pub const TRIAL_MARKER: &str = "TRIAL:";

/// One `main.rs`: player items at the top, the trial inside `main`.
///
/// Player code defines items (`fn`, `struct`, …) — no `main` of its
/// own. Lints that only exist because the trial may not exercise every
/// item are allowed; correctness lints stay on.
///
/// The harness times the trial itself and prints the elapsed millis
/// after the pass marker — so local rustc and the far playground use
/// the same stopwatch, and it measures pure trial execution (no
/// compile time, no network).
pub fn compose(puzzle: &Puzzle, player_code: &str) -> String {
    format!(
        "#![allow(dead_code, unused_variables, unused_mut)]\n\
         // ── player spell ──────────────────────────────────────\n\
         {player_code}\n\
         // ── hidden trial ──────────────────────────────────────\n\
         fn main() {{\n\
         let __bb_clock = std::time::Instant::now();\n\
         {trial}\n\
         println!(\"{PASS_MARKER} {{}}\", __bb_clock.elapsed().as_millis());\n\
         }}\n",
        trial = puzzle.trial,
    )
}

/// Millis printed after the pass marker, when present and sane.
pub fn parse_trial_millis(stdout: &str) -> u64 {
    stdout
        .find(PASS_MARKER)
        .map(|pos| &stdout[pos + PASS_MARKER.len()..])
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|token| token.parse().ok())
        .unwrap_or(0)
}
