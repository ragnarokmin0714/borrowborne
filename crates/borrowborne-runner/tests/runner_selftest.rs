//! End-to-end selftest of the rustc backend against a real puzzle:
//! good code passes, a borrow error is blocked, a wrong answer fails
//! the trial, and a player panic is lethal.
//!
//! Requires `rustc` on PATH — true wherever this workspace builds.

use borrowborne_core::{Puzzle, Verdict};
use borrowborne_runner::{RustcLocal, Sandbox};

fn kept_key_puzzle() -> Puzzle {
    Puzzle {
        id: "test-kept-key".into(),
        title: "The Kept Key".into(),
        scene: String::new(),
        concepts: vec![],
        starter_code: String::new(),
        trial: r#"assert!(
    open_gate(String::from("rusty-key")),
    "TRIAL: the gate stayed shut for the rightful key"
);"#
        .into(),
        hints: vec![],
        solution: String::new(),
    }
}

#[test]
fn correct_spell_passes() {
    let code = r#"
fn inspect(key: &str) -> bool { key == "rusty-key" }
fn open_gate(key: String) -> bool {
    let approved = inspect(&key);
    approved && key == "rusty-key"
}
"#;
    let v = RustcLocal.evaluate(&kept_key_puzzle(), code);
    assert_eq!(v, Verdict::Passed);
}

#[test]
fn borrow_error_is_blocked_by_the_world() {
    let code = r#"
fn inspect(key: String) -> bool { key == "rusty-key" }
fn open_gate(key: String) -> bool {
    let approved = inspect(key);
    approved && key == "rusty-key" // use after move
}
"#;
    match RustcLocal.evaluate(&kept_key_puzzle(), code) {
        Verdict::CompileError(msg) => {
            assert!(
                msg.contains("E0382"),
                "expected a move diagnostic, got: {msg}"
            )
        }
        other => panic!("expected CompileError, got {other:?}"),
    }
}

#[test]
fn wrong_answer_fails_the_trial() {
    let code = r#"
fn open_gate(_key: String) -> bool { false } // gate never opens
"#;
    match RustcLocal.evaluate(&kept_key_puzzle(), code) {
        Verdict::TrialFailed(msg) => assert!(msg.contains("rightful key"), "got: {msg}"),
        other => panic!("expected TrialFailed, got {other:?}"),
    }
}

#[test]
fn player_panic_is_permadeath() {
    let code = r#"
fn open_gate(_key: String) -> bool {
    let cursed: Option<u32> = None;
    cursed.unwrap(); // the classic sin
    true
}
"#;
    match RustcLocal.evaluate(&kept_key_puzzle(), code) {
        Verdict::Panicked(_) => {}
        other => panic!("expected Panicked, got {other:?}"),
    }
}
