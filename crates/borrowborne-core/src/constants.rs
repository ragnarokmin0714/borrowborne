//! Centralized constants and tunables.
//!
//! The single place to change the game's identity or difficulty
//! tuning. Nothing else in the codebase hard-codes these values, so a
//! rename or a balance tweak is a one-line change.

/// Product name. Shown in the window title and used for config paths.
pub const APP_NAME: &str = "Borrowborne";

/// One-line tagline shown in the UI header.
pub const APP_TAGLINE: &str = "Fear the old blood. Respect the borrow checker.";

/// Folder name under the OS config directory for saves.
pub const CONFIG_DIR_NAME: &str = "borrowborne";

/// Directory (relative to the workspace root) holding chapter files.
pub const CHAPTERS_DIR: &str = "content/chapters";

/// Extension for chapter content files.
pub const CHAPTER_FILE_EXT: &str = "ron";

/// Wall-clock budget for compiling one spell. `rustc` on a tiny file is
/// well under a second; the margin covers cold disk caches.
pub const COMPILE_TIMEOUT_SECS: u64 = 30;

/// Wall-clock budget for running the compiled trial. Anything longer is
/// treated as an infinite loop and judged [`crate::Verdict::Timeout`].
pub const RUN_TIMEOUT_SECS: u64 = 5;

/// How many deaths a run tolerates before the roguelike reset. Kept
/// generous while the content pool is small.
pub const LIVES_PER_RUN: u32 = 7;
