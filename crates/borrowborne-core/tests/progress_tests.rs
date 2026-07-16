//! Progress rules: passes teach, panics kill, runs reset — and saves
//! stay small and self-healing. Echo rules follow Bloodborne: drop on
//! death, reclaim by re-solving, lose the old stain to a new one.

use borrowborne_core::constants::{ECHOES_PER_SOLVE, LIVES_PER_RUN, STARTING_ECHOES};
use borrowborne_core::{Bloodstain, Chapter, Concept, Curriculum, Progress, Puzzle, Verdict};

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
                hints: vec![],
                solution: String::new(),
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
fn echoes_flow_earn_once_drop_reclaim_replace() {
    let mut p = Progress::default();
    assert_eq!(p.echoes, STARTING_ECHOES);

    // First solve pays; solving the same gate again does not.
    p.record("of-01", &[], &Verdict::Passed);
    assert_eq!(p.echoes, STARTING_ECHOES + ECHOES_PER_SOLVE);
    p.record("of-01", &[], &Verdict::Passed);
    assert_eq!(p.echoes, STARTING_ECHOES + ECHOES_PER_SOLVE);

    // Death drops the whole purse where the hunter fell.
    let held = p.echoes;
    p.record("of-02", &[], &Verdict::Panicked("boom".into()));
    assert_eq!(p.echoes, 0);
    assert_eq!(
        p.bloodstain,
        Some(Bloodstain {
            puzzle_id: "of-02".into(),
            amount: held
        })
    );

    // Dying empty-handed does not erase the stain.
    p.record("of-03", &[], &Verdict::Panicked("boom".into()));
    assert_eq!(p.bloodstain.as_ref().unwrap().puzzle_id, "of-02");

    // The corpse run: solving where you fell reclaims the echoes
    // (plus the first-solve reward for the new gate).
    p.record("of-02", &[], &Verdict::Passed);
    assert_eq!(p.echoes, held + ECHOES_PER_SOLVE);
    assert_eq!(p.bloodstain, None);

    // A new death holding echoes replaces any older stain.
    p.record("of-04", &[], &Verdict::Panicked("boom".into()));
    p.record("of-01", &[], &Verdict::Passed); // reopen: no pay, no reclaim
    assert_eq!(p.echoes, 0);
    p.record("of-05", &[], &Verdict::Passed); // earn fresh echoes
    p.record("of-05", &[], &Verdict::Panicked("boom".into()));
    let stain = p.bloodstain.as_ref().unwrap();
    assert_eq!(stain.puzzle_id, "of-05", "new stain must replace the old");
    assert_eq!(stain.amount, ECHOES_PER_SOLVE, "old echoes are lost");
}

#[test]
fn hints_cost_echoes_and_refuse_the_broke() {
    let mut p = Progress::default(); // 30 echoes
    assert!(p.buy_hint(0)); // -5
    assert!(p.buy_hint(1)); // -10
    assert_eq!(p.echoes, STARTING_ECHOES - 5 - 10);
    assert!(!p.buy_hint(2), "20 > 15: the lantern refuses");
    assert_eq!(p.echoes, 15, "a refused purchase deducts nothing");
}

#[test]
fn rebuild_returns_echoes_from_a_stale_stain() {
    let mut p = Progress {
        echoes: 10,
        bloodstain: Some(Bloodstain {
            puzzle_id: "deleted-puzzle".into(),
            amount: 40,
        }),
        ..Progress::default()
    };
    p.rebuild(&tiny_curriculum());
    assert_eq!(p.bloodstain, None);
    assert_eq!(p.echoes, 50, "content edits must never steal echoes");
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
        ..Progress::default()
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
fn chapters_unseal_at_the_threshold() {
    // Ten puzzles in chapter 0, one in chapter 1.
    let mk = |id: &str| Puzzle {
        id: id.into(),
        title: String::new(),
        scene: String::new(),
        concepts: vec![Concept::Variables],
        starter_code: String::new(),
        trial: "TRIAL:".into(),
        hints: vec![],
        solution: String::new(),
    };
    let cur = Curriculum {
        chapters: vec![
            Chapter {
                id: "a".into(),
                name: String::new(),
                tagline: String::new(),
                puzzles: (0..10).map(|i| mk(&format!("a-{i}"))).collect(),
            },
            Chapter {
                id: "b".into(),
                name: String::new(),
                tagline: String::new(),
                puzzles: vec![mk("b-0")],
            },
        ],
    };

    let mut p = Progress::default();
    assert!(p.chapter_unlocked(&cur, 0), "the first region is open");
    assert!(!p.chapter_unlocked(&cur, 1));

    for i in 0..6 {
        p.record(&format!("a-{i}"), &[], &Verdict::Passed);
    }
    assert!(!p.chapter_unlocked(&cur, 1), "6/10 < 70%");
    p.record("a-6", &[], &Verdict::Passed);
    assert!(p.chapter_unlocked(&cur, 1), "7/10 breaks the seal");
    assert!(!p.chapter_unlocked(&cur, 2), "past the end stays sealed");
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
