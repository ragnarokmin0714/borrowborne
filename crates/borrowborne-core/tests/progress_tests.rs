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
                toolbox: vec![],
                solution: String::new(),
            }],
        }],
    }
}

#[test]
fn pass_records_solved_and_learned() {
    let mut p = Progress::default();
    let ended = p.record(
        "of-01",
        &[Concept::Move, Concept::Borrow],
        &Verdict::Passed { trial_millis: 0 },
    );
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
    p.record(
        "of-01",
        &[],
        &Verdict::Passed {
            trial_millis: 900, /* grade B: no bonus */
        },
    );
    assert_eq!(p.echoes, STARTING_ECHOES + ECHOES_PER_SOLVE);
    p.record(
        "of-01",
        &[],
        &Verdict::Passed {
            trial_millis: 900, /* grade B: no bonus */
        },
    );
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
    p.record(
        "of-02",
        &[],
        &Verdict::Passed {
            trial_millis: 900, /* grade B: no bonus */
        },
    );
    assert_eq!(p.echoes, held + ECHOES_PER_SOLVE);
    assert_eq!(p.bloodstain, None);

    // A new death holding echoes replaces any older stain.
    p.record("of-04", &[], &Verdict::Panicked("boom".into()));
    p.record(
        "of-01",
        &[],
        &Verdict::Passed {
            trial_millis: 900, /* grade B: no bonus */
        },
    ); // reopen: no pay, no reclaim
    assert_eq!(p.echoes, 0);
    p.record(
        "of-05",
        &[],
        &Verdict::Passed {
            trial_millis: 900, /* grade B: no bonus */
        },
    ); // earn fresh echoes
    p.record("of-05", &[], &Verdict::Panicked("boom".into()));
    let stain = p.bloodstain.as_ref().unwrap();
    assert_eq!(stain.puzzle_id, "of-05", "new stain must replace the old");
    assert_eq!(stain.amount, ECHOES_PER_SOLVE, "old echoes are lost");
}

#[test]
fn speed_grades_pay_bonuses_and_keep_the_best() {
    use borrowborne_core::constants::{ECHOES_PER_SOLVE, ECHO_BONUS_S};
    use borrowborne_core::Grade;

    let mut p = Progress::default();
    // A slow first solve: base pay, grade B on record.
    p.record("slow", &[], &Verdict::Passed { trial_millis: 900 });
    assert_eq!(p.echoes, STARTING_ECHOES + ECHOES_PER_SOLVE);
    assert_eq!(p.grades["slow"], Grade::B);

    // A blazing first solve: base + S bonus.
    p.record("fast", &[], &Verdict::Passed { trial_millis: 3 });
    assert_eq!(
        p.echoes,
        STARTING_ECHOES + ECHOES_PER_SOLVE * 2 + ECHO_BONUS_S
    );
    assert_eq!(p.grades["fast"], Grade::S);

    // Re-solving faster upgrades the grade but pays nothing more.
    let purse = p.echoes;
    p.record("slow", &[], &Verdict::Passed { trial_millis: 10 });
    assert_eq!(p.echoes, purse, "an open gate pays nothing");
    assert_eq!(p.grades["slow"], Grade::S, "the best grade is kept");

    // Re-solving slower never downgrades.
    p.record("fast", &[], &Verdict::Passed { trial_millis: 999 });
    assert_eq!(p.grades["fast"], Grade::S);
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
fn easy_mode_makes_hints_free_and_unrefusable() {
    use borrowborne_core::Difficulty;
    let mut p = Progress {
        echoes: 0, // broke: Normal would refuse every tier
        difficulty: Difficulty::Easy,
        ..Progress::default()
    };
    for tier in 0..3 {
        assert_eq!(p.hint_cost(tier), 0, "Easy: every tier is free");
        assert!(p.buy_hint(tier), "Easy: the lantern never refuses");
    }
    assert_eq!(p.echoes, 0, "free hints deduct nothing");

    // Difficulty rides along in the save and defaults to Normal.
    let old: Progress = serde_json::from_str(r#"{"solved":[],"deaths":0,"total_deaths":0}"#)
        .expect("old save must load");
    assert_eq!(old.difficulty, Difficulty::Normal);
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
        &Verdict::Passed { trial_millis: 0 },
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
fn hunter_names_default_and_sanitize() {
    // Empty means "still the nameless outlander" — the app renders a
    // localized default; the save never stores one language's word.
    let mut p = Progress::default();
    assert_eq!(p.hunter_name, "");

    // Whitespace collapses back to nameless.
    p.hunter_name = "   ".into();
    p.sanitize_name();
    assert_eq!(p.hunter_name, "");

    // Overlong names are clipped, not rejected.
    p.hunter_name = "x".repeat(100);
    p.sanitize_name();
    assert_eq!(p.hunter_name.chars().count(), 24);

    // Old saves without the field load as nameless…
    let old: Progress = serde_json::from_str(r#"{"solved":[],"deaths":0,"total_deaths":0}"#)
        .expect("old save must load");
    assert_eq!(old.hunter_name, "");

    // …and ones that stored the pre-outlander "Good Hunter" migrate
    // back to nameless, so they too get the localized default.
    p.hunter_name = "Good Hunter".into();
    p.sanitize_name();
    assert_eq!(p.hunter_name, "");
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
        toolbox: vec![],
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
        p.record(&format!("a-{i}"), &[], &Verdict::Passed { trial_millis: 0 });
    }
    assert!(!p.chapter_unlocked(&cur, 1), "6/10 < 70%");
    p.record("a-6", &[], &Verdict::Passed { trial_millis: 0 });
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
