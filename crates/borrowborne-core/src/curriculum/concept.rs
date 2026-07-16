//! Rust concepts as skill-tree nodes.

use serde::{Deserialize, Serialize};

/// A Rust concept a puzzle can teach. One variant per node in the
/// journal's skill tree; chapters group them, puzzles light them up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Concept {
    // Newbie Village
    Variables,
    Mutability,
    ControlFlow,
    // Ownership Forest
    Move,
    Borrow,
    MutBorrow,
    // Enum Town
    Structs,
    Enums,
    Match,
    // Result Swamp
    OptionType,
    ResultType,
    QuestionMark,
    // Trait Guild
    Traits,
    Generics,
    // Iterator Library
    Collections,
    Iterators,
    // Lifetime Shrine
    Lifetimes,
    // Concurrency Keep
    Threads,
    Channels,
    SharedState,
}
