//! Core error type.

use thiserror::Error;

/// Errors from loading or validating game content.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("failed to read chapter file {path}: {source}")]
    ChapterRead {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to parse chapter file {path}: {source}")]
    ChapterParse {
        path: String,
        source: ron::error::SpannedError,
    },

    #[error("no chapter files found in {0}")]
    NoChapters(String),
}
