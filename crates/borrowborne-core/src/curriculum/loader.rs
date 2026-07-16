//! Chapter file parsing.

use std::path::Path;

use crate::constants::CHAPTER_FILE_EXT;
use crate::error::CoreError;

use super::{Chapter, Curriculum};

/// Parse one chapter from RON text. `path` is only used for error text.
pub fn parse_chapter(text: &str, path: &str) -> Result<Chapter, CoreError> {
    ron::from_str(text).map_err(|source| CoreError::ChapterParse {
        path: path.to_owned(),
        source,
    })
}

/// Load every `*.ron` chapter in a directory, sorted by file name so
/// the `01-`, `02-` prefixes define learning order.
pub fn load_dir(dir: &Path) -> Result<Curriculum, CoreError> {
    let read_err = |source| CoreError::ChapterRead {
        path: dir.display().to_string(),
        source,
    };

    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .map_err(read_err)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == CHAPTER_FILE_EXT))
        .collect();
    paths.sort();

    let mut chapters = Vec::new();
    for path in paths {
        let shown = path.display().to_string();
        let text = std::fs::read_to_string(&path).map_err(|source| CoreError::ChapterRead {
            path: shown.clone(),
            source,
        })?;
        chapters.push(parse_chapter(&text, &shown)?);
    }

    if chapters.is_empty() {
        return Err(CoreError::NoChapters(dir.display().to_string()));
    }
    Ok(Curriculum { chapters })
}
