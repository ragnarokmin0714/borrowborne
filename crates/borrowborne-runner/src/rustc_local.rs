//! MVP sandbox backend: shell out to the local `rustc`.
//!
//! One temp directory per cast: write the composed `main.rs`, compile,
//! run with a wall-clock budget, then map the outcome onto
//! [`Verdict`]. Compiler stderr is passed through verbatim — the app
//! performs it as the voice of the world.

use std::path::PathBuf;
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use borrowborne_core::constants::{COMPILE_TIMEOUT_SECS, RUN_TIMEOUT_SECS};
use borrowborne_core::{Puzzle, Verdict};

use crate::harness::{compose, parse_trial_millis, PASS_MARKER, TRIAL_MARKER};
use crate::Sandbox;

/// Judges spells with the machine's own `rustc`.
#[derive(Default)]
pub struct RustcLocal;

/// Distinguishes concurrent casts (and reruns within one process).
static CAST_SEQ: AtomicU64 = AtomicU64::new(0);

impl Sandbox for RustcLocal {
    fn evaluate(&self, puzzle: &Puzzle, player_code: &str) -> Verdict {
        let dir = cast_dir();
        let verdict = run_cast(&dir, puzzle, player_code);
        let _ = std::fs::remove_dir_all(&dir); // best-effort cleanup
        verdict
    }
}

fn cast_dir() -> PathBuf {
    let seq = CAST_SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("borrowborne-cast-{}-{seq}", std::process::id()))
}

fn run_cast(dir: &PathBuf, puzzle: &Puzzle, player_code: &str) -> Verdict {
    if let Err(e) = std::fs::create_dir_all(dir) {
        return Verdict::CompileError(format!("could not prepare the ritual site: {e}"));
    }
    let src = dir.join("main.rs");
    // Explicit .exe on Windows: `-o` names and later execution both stay
    // unambiguous, whatever rustc's suffix behavior.
    let bin = dir.join(if cfg!(windows) { "spell.exe" } else { "spell" });
    if let Err(e) = std::fs::write(&src, compose(puzzle, player_code)) {
        return Verdict::CompileError(format!("could not inscribe the spell: {e}"));
    }

    // Compile.
    let compile = Command::new("rustc")
        .arg("--edition=2021")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let output = match compile {
        Ok(child) => match wait_timeout(child, Duration::from_secs(COMPILE_TIMEOUT_SECS)) {
            Some(out) => out,
            None => return Verdict::Timeout,
        },
        Err(e) => return Verdict::CompileError(format!("rustc could not be summoned: {e}")),
    };
    if !output.status.success() {
        return Verdict::CompileError(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    // Run the trial.
    let run = Command::new(&bin)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let output = match run {
        Ok(child) => match wait_timeout(child, Duration::from_secs(RUN_TIMEOUT_SECS)) {
            Some(out) => out,
            None => return Verdict::Timeout,
        },
        Err(e) => return Verdict::Panicked(format!("the spell fizzled before it began: {e}")),
    };

    judge_run(&output)
}

fn judge_run(output: &Output) -> Verdict {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() && stdout.contains(PASS_MARKER) {
        return Verdict::Passed {
            trial_millis: parse_trial_millis(&stdout),
        };
    }
    // A panic whose message carries the trial marker is a failed trial;
    // any other abnormal end is the player's own doing. Permadeath.
    if let Some(pos) = stderr.find(TRIAL_MARKER) {
        let msg = stderr[pos + TRIAL_MARKER.len()..]
            .lines()
            .next()
            .unwrap_or("")
            .trim();
        return Verdict::TrialFailed(msg.to_owned());
    }
    Verdict::Panicked(stderr.into_owned())
}

/// Wait for a child with a wall-clock budget; `None` means it was
/// killed for overstaying. Output pipes are small (assert messages),
/// so collecting them after exit cannot deadlock.
fn wait_timeout(mut child: Child, budget: Duration) -> Option<Output> {
    let deadline = Instant::now() + budget;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return child.wait_with_output().ok(),
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait(); // reap; discard output of a killed run
                return None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(30)),
            Err(_) => return None,
        }
    }
}
