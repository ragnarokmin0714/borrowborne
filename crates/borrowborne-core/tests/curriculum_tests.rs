//! Content guards: every chapter file parses and follows the rules the
//! runner relies on (unique ids, concepts present, TRIAL markers).

use std::collections::HashSet;
use std::path::PathBuf;

use borrowborne_core::curriculum::load_dir;
use borrowborne_core::Curriculum;

fn chapters_dir() -> PathBuf {
    // crates/borrowborne-core → workspace root → content/chapters
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/chapters")
}

fn curriculum() -> Curriculum {
    load_dir(&chapters_dir()).expect("chapter files must parse")
}

#[test]
fn all_chapters_parse() {
    let cur = curriculum();
    assert!(!cur.chapters.is_empty());
    assert!(cur.puzzle_count() > 0);
}

#[test]
fn puzzle_ids_are_unique() {
    let cur = curriculum();
    let mut seen = HashSet::new();
    for ch in &cur.chapters {
        for p in &ch.puzzles {
            assert!(seen.insert(p.id.clone()), "duplicate puzzle id {}", p.id);
        }
    }
}

#[test]
fn every_puzzle_teaches_and_tests() {
    let cur = curriculum();
    for ch in &cur.chapters {
        for p in &ch.puzzles {
            assert!(!p.concepts.is_empty(), "{} teaches nothing", p.id);
            assert!(!p.starter_code.trim().is_empty(), "{} has no starter", p.id);
            // The runner tells TrialFailed from Panicked by this marker.
            assert!(
                p.trial.contains("TRIAL:"),
                "{} trial lacks a TRIAL: marker",
                p.id
            );
        }
    }
}
