//! The content gate: every shipped puzzle must be provably broken and
//! provably solvable, judged by the real `rustc`.
//!
//! - the starter code must NOT pass (otherwise the door is already
//!   open and the puzzle teaches nothing),
//! - the canonical `solution` MUST pass (otherwise the door can never
//!   open and the player is stuck through no fault of their own).
//!
//! Slow by design (two compiles per puzzle, and the Sleepless Wheel's
//! starter deliberately runs into the timeout) — this is the test that
//! makes `.ron` edits safe.

use std::path::PathBuf;

use borrowborne_core::curriculum::load_dir;
use borrowborne_core::Verdict;
use borrowborne_runner::{RustcLocal, Sandbox};

fn chapters_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/chapters")
}

#[test]
fn every_starter_fails_and_every_solution_passes() {
    let curriculum = load_dir(&chapters_dir()).expect("chapters must load");

    for chapter in &curriculum.chapters {
        for puzzle in &chapter.puzzles {
            assert!(
                !puzzle.solution.trim().is_empty(),
                "{}: missing canonical solution",
                puzzle.id
            );

            let starter = RustcLocal.evaluate(puzzle, &puzzle.starter_code);
            assert_ne!(
                starter,
                Verdict::Passed,
                "{}: the starter already passes — nothing to solve",
                puzzle.id
            );

            let solution = RustcLocal.evaluate(puzzle, &puzzle.solution);
            assert_eq!(
                solution,
                Verdict::Passed,
                "{}: the canonical solution does not pass",
                puzzle.id
            );
        }
    }
}
