//! Progress rules: passes teach, panics kill, runs reset — and saves
//! stay small and self-healing.

use borrowborne_core::constants::LIVES_PER_RUN;
use borrowborne_core::{Chapter, Concept, Curriculum, Progress, Puzzle, Verdict};

/// A one-chapter curriculum with a single known puzzle.
fn tiny_curriculum() -> Curriculum {
    Curriculum {
        chapters: vec![Chapter {
            id: "test-chapter".into(),
            name: String::new(),
            tagline: String::new(),
            puzzles: vec![Puzzle {
                id: "known-puzzle".into(),
                title: String::new(),
                scene: String::new(),
                concepts: vec![Concept::Move, Concept::Borrow],
                starter_code: String::new(),
                trial: "TRIAL:".into(),
            }],
        }],
    }
}

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
fn rebuild_drops_stale_ids_and_recomputes_learned() {
    let mut p = Progress {
        solved: ["known-puzzle".to_owned(), "deleted-puzzle".to_owned()]
            .into_iter()
            .collect(),
        learned: Default::default(), // as after deserialization: empty
        deaths: 999,                 // save damage
        total_deaths: 3,
    };
    p.rebuild(&tiny_curriculum());

    assert_eq!(p.solved.len(), 1, "stale id must be dropped");
    assert!(p.solved.contains("known-puzzle"));
    assert!(p.learned.contains(&Concept::Move) && p.learned.contains(&Concept::Borrow));
    assert!(p.deaths < LIVES_PER_RUN, "deaths must be clamped");
    assert!(p.total_deaths >= p.deaths);
}

#[test]
fn save_format_stays_small_learned_is_not_serialized() {
    let mut p = Progress::default();
    p.record(
        "known-puzzle",
        &[Concept::Move, Concept::Borrow],
        &Verdict::Passed,
    );

    let json = serde_json::to_string(&p).unwrap();
    assert!(
        !json.contains("learned") && !json.contains("Move"),
        "derived data must not reach the save: {json}"
    );

    // Round trip + rebuild restores what was skipped.
    let mut back: Progress = serde_json::from_str(&json).unwrap();
    assert!(back.learned.is_empty());
    back.rebuild(&tiny_curriculum());
    assert!(back.learned.contains(&Concept::Move));
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
