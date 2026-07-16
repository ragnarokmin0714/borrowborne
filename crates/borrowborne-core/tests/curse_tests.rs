//! Curse rules and content guards: the shipped curse book parses,
//! refusals hit only what they name, taxes tax.

use std::path::PathBuf;

use borrowborne_core::{CurseBook, CurseEffect};

fn shipped_book() -> CurseBook {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/curses.ron");
    let text = std::fs::read_to_string(&path).expect("curses.ron must exist");
    CurseBook::parse(&text, "content/curses.ron").expect("curses.ron must parse")
}

#[test]
fn shipped_curses_parse_and_have_unique_ids() {
    let book = shipped_book();
    assert!(!book.curses.is_empty());
    let mut ids: Vec<_> = book.curses.iter().map(|c| c.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), book.curses.len(), "duplicate curse id");
}

#[test]
fn forbid_snippet_refuses_only_what_it_names() {
    let book = shipped_book();
    let twinless = book.get("curse-of-the-twinless").expect("known curse");
    assert!(twinless.refusal("let x = key.clone();").is_some());
    assert!(twinless.refusal("let x = &key;").is_none());
    assert_eq!(twinless.cast_tax(), 0);
}

#[test]
fn echo_tax_taxes_and_never_refuses() {
    let book = shipped_book();
    let poverty = book.get("curse-of-poverty").expect("known curse");
    assert!(matches!(poverty.effect, CurseEffect::EchoTax(2)));
    assert_eq!(poverty.cast_tax(), 2);
    assert!(poverty.refusal("anything.clone().unwrap()").is_none());
}

#[test]
fn roll_is_total_over_the_book() {
    let book = shipped_book();
    for seed in 0..(book.curses.len() as u64 * 3) {
        assert!(book.roll(seed).is_some());
    }
    let empty = CurseBook { curses: vec![] };
    assert!(empty.roll(7).is_none());
}
