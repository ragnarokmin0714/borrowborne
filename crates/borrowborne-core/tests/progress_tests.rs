//! Progress rules: passes teach, panics kill, runs reset.

use borrowborne_core::constants::LIVES_PER_RUN;
use borrowborne_core::{Concept, Progress, Verdict};

#[test]
fn pass_records_solved_and_learned() {
    let mut p = Progress::default();
    let ended = p.record("of-01", &[Concept::Move, Concept::Borrow], &Verdict::Passed);
    assert!(!ended);
    assert!(p.solved.contains("of-01"));
    assert!(p.learned.contains(&Concept::Move));
    assert_eq!(p.deaths, 0);
}

#[test]
fn compile_error_is_not_lethal() {
    let mut p = Progress::default();
    p.record("of-01", &[], &Verdict::CompileError("E0382".into()));
    assert_eq!(p.deaths, 0);
    assert!(p.solved.is_empty());
}

#[test]
fn panic_costs_a_life_and_run_resets() {
    let mut p = Progress::default();
    for i in 0..LIVES_PER_RUN - 1 {
        let ended = p.record("of-01", &[], &Verdict::Panicked("boom".into()));
        assert!(!ended, "run ended too early at death {i}");
    }
    // The final death ends the run; the counter resets, tombstones stay.
    let ended = p.record("of-01", &[], &Verdict::Panicked("boom".into()));
    assert!(ended);
    assert_eq!(p.deaths, 0);
    assert_eq!(p.total_deaths, LIVES_PER_RUN);
    assert_eq!(p.lives_left(), LIVES_PER_RUN);
}
