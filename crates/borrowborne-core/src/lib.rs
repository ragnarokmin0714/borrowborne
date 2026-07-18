//! Borrowborne core: pure game and curriculum logic.
//!
//! No UI, no processes, no filesystem side effects beyond loading
//! content. Everything here is plain data so the front end (and its
//! tests) can stay thin.

pub mod constants;
pub mod curriculum;
pub mod curse;
pub mod error;
pub mod progress;
pub mod verdict;

pub use curriculum::{Chapter, Concept, Curriculum, Puzzle};
pub use curse::{Curse, CurseBook, CurseEffect};
pub use error::CoreError;
pub use progress::{Bloodstain, Difficulty, Progress};
pub use verdict::{Grade, Verdict};
