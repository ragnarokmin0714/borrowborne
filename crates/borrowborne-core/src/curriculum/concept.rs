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
    // Algorithm Dungeon (hardcore) — complexity, not a language feature.
    Algorithms,
}

impl Concept {
    /// Short Rust-term label for the journal's skill node. Left in
    /// English on purpose: these are Rust's own terms (`&mut`, `match`,
    /// `Arc<Mutex>`), the same in every UI language.
    pub fn label(self) -> &'static str {
        match self {
            Concept::Variables => "variables",
            Concept::Mutability => "mut",
            Concept::ControlFlow => "control flow",
            Concept::Move => "move",
            Concept::Borrow => "& borrow",
            Concept::MutBorrow => "&mut",
            Concept::Structs => "struct",
            Concept::Enums => "enum",
            Concept::Match => "match",
            Concept::OptionType => "Option",
            Concept::ResultType => "Result",
            Concept::QuestionMark => "? operator",
            Concept::Traits => "trait",
            Concept::Generics => "generics",
            Concept::Collections => "collections",
            Concept::Iterators => "iterators",
            Concept::Lifetimes => "lifetimes",
            Concept::Threads => "threads",
            Concept::Channels => "channels",
            Concept::SharedState => "Arc<Mutex>",
            Concept::Algorithms => "algorithms",
        }
    }
}
