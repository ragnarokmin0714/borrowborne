//! The far judge: the official Rust Playground's execute API.
//!
//! Used by the web build, where no local `rustc` exists. This module is
//! pure — it builds the request body and judges the response; actually
//! sending HTTP is the app's job (native threads vs. browser fetch
//! differ too much to hide here).

use borrowborne_core::{Puzzle, Verdict};
use serde::Deserialize;

use crate::harness::{compose, PASS_MARKER, TRIAL_MARKER};

/// Playground execute endpoint. Serves permissive CORS — the same API
/// mdBook's runnable examples call from arbitrary origins.
pub const EXECUTE_URL: &str = "https://play.rust-lang.org/execute";

/// JSON body for one cast.
pub fn request_body(puzzle: &Puzzle, player_code: &str) -> Vec<u8> {
    serde_json::json!({
        "channel": "stable",
        "mode": "debug",
        "edition": "2021",
        "crateType": "bin",
        "tests": false,
        "backtrace": false,
        "code": compose(puzzle, player_code),
    })
    .to_string()
    .into_bytes()
}

#[derive(Deserialize)]
struct ExecuteResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

/// Judge the playground's answer.
///
/// The playground runs compile+run as one phase, so ordering does the
/// separation: the pass marker proves the trial ran clean, the trial
/// marker names a failed trial, a panic line is the player's own
/// doing, and anything else is the compiler refusing.
pub fn parse_response(status: u16, bytes: &[u8]) -> Verdict {
    if status != 200 {
        return Verdict::CompileError(format!("the far judge answered with status {status}"));
    }
    let resp: ExecuteResponse = match serde_json::from_slice(bytes) {
        Ok(r) => r,
        Err(e) => return Verdict::CompileError(format!("the far judge spoke in tongues: {e}")),
    };

    if resp.success && resp.stdout.contains(PASS_MARKER) {
        return Verdict::Passed;
    }
    if let Some(pos) = resp.stderr.find(TRIAL_MARKER) {
        let msg = resp.stderr[pos + TRIAL_MARKER.len()..]
            .lines()
            .next()
            .unwrap_or("")
            .trim();
        return Verdict::TrialFailed(msg.to_owned());
    }
    if resp.stderr.contains("panicked at") {
        return Verdict::Panicked(resp.stderr);
    }
    if resp.stderr.contains("timed out") {
        return Verdict::Timeout;
    }
    Verdict::CompileError(resp.stderr)
}
