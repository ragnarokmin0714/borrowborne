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

/// Lines the harness prepends before the player's first line in the
/// composed file (the allow attribute and the banner comment). Keep in
/// sync with [`compose`].
pub const PLAYER_LINE_OFFSET: usize = 2;

/// Map the first `<file>.rs:<line>:` location in a diagnostic back to
/// a line of the player's own code (1-based). `None` when the
/// diagnostic names no location, or when it points into harness or
/// trial territory — a wrong line is worse than no line.
///
/// Works on both judges' output: local rustc (`--> /tmp/…/spell.rs:5:9`,
/// `panicked at spell.rs:5:9`) and the playground (`src/main.rs`).
pub fn player_error_line(diagnostic: &str, player_lines: usize) -> Option<usize> {
    let pos = diagnostic.find(".rs:")?;
    let rest = &diagnostic[pos + ".rs:".len()..];
    let digits_end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    let composed: usize = rest[..digits_end].parse().ok()?;
    let player = composed.checked_sub(PLAYER_LINE_OFFSET)?;
    (1..=player_lines).contains(&player).then_some(player)
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

#[cfg(test)]
mod tests {
    use super::player_error_line;

    #[test]
    fn maps_rustc_and_playground_locations_to_player_lines() {
        // Composed line 5 = player line 3 (two harness lines above).
        let local = "error[E0308]: mismatched types\n --> /tmp/x/spell.rs:5:9\n";
        assert_eq!(player_error_line(local, 10), Some(3));
        let web = "error[E0599]: no method\n --> src/main.rs:5:9\n";
        assert_eq!(player_error_line(web, 10), Some(3));
        let panic = "thread 'main' panicked at src/main.rs:4:12:\nboom";
        assert_eq!(player_error_line(panic, 10), Some(2));
    }

    #[test]
    fn refuses_locations_outside_the_player_code() {
        // Harness territory (composed lines 1–2)…
        assert_eq!(player_error_line("--> spell.rs:2:1", 10), None);
        // …trial territory (beyond the player's last line)…
        assert_eq!(player_error_line("--> spell.rs:9:1", 3), None);
        // …and diagnostics with no location at all.
        assert_eq!(player_error_line("error: expected `;`", 10), None);
    }
}
