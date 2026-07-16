//! Borrowborne runner: the dangerous edge.
//!
//! This is the only crate allowed to touch a compiler or spawn a
//! process. Everything hides behind [`Sandbox`], so the MVP local
//! `rustc` backend can be swapped for a wasm32 + wasmtime sandbox
//! without the app noticing.
//!
//! Security note: [`RustcLocal`] executes player-written native code on
//! the local machine. Acceptable for a single-player learning game run
//! on yourself; the wasm backend is the real answer before third-party
//! content.

mod harness;
mod rustc_local;

pub use rustc_local::RustcLocal;

use borrowborne_core::{Puzzle, Verdict};

/// Judges a spell: compiles the player's code with the puzzle's hidden
/// trial and reports what the world said.
pub trait Sandbox {
    fn evaluate(&self, puzzle: &Puzzle, player_code: &str) -> Verdict;
}
