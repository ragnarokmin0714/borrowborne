//! Pure tests for the far-judge protocol: canned playground responses
//! in, verdicts out. No network involved.

use borrowborne_core::Verdict;
use borrowborne_runner::playground::parse_response;

fn body(success: bool, stdout: &str, stderr: &str) -> Vec<u8> {
    serde_json::json!({ "success": success, "stdout": stdout, "stderr": stderr })
        .to_string()
        .into_bytes()
}

#[test]
fn pass_marker_wins() {
    let v = parse_response(200, &body(true, "__BORROWBORNE_PASS__\n", ""));
    assert_eq!(v, Verdict::Passed);
}

#[test]
fn trial_marker_beats_panic_text() {
    let stderr = "thread 'main' panicked at src/main.rs:9:5:\nTRIAL: the gate stayed shut\n";
    match parse_response(200, &body(false, "", stderr)) {
        Verdict::TrialFailed(msg) => assert_eq!(msg, "the gate stayed shut"),
        other => panic!("expected TrialFailed, got {other:?}"),
    }
}

#[test]
fn bare_panic_is_permadeath() {
    let stderr = "thread 'main' panicked at src/main.rs:3:12:\ncalled `Option::unwrap()` on a `None` value\n";
    assert!(matches!(
        parse_response(200, &body(false, "", stderr)),
        Verdict::Panicked(_)
    ));
}

#[test]
fn diagnostics_are_compile_errors() {
    let stderr = "error[E0382]: borrow of moved value: `key`\n";
    match parse_response(200, &body(false, "", stderr)) {
        Verdict::CompileError(msg) => assert!(msg.contains("E0382")),
        other => panic!("expected CompileError, got {other:?}"),
    }
}

#[test]
fn garbage_and_bad_status_stay_graceful() {
    assert!(matches!(
        parse_response(502, b"Bad Gateway"),
        Verdict::CompileError(_)
    ));
    assert!(matches!(
        parse_response(200, b"not json"),
        Verdict::CompileError(_)
    ));
}
