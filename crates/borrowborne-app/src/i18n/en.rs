use super::{Tr, Voice};

pub static EN: Tr = Tr {
    tagline: "Fear the old blood. Respect the borrow checker.",
    language: "Language",
    lives: "Lives",
    progress: "Progress",
    deaths_total: "tombstones",

    cast: "⚔ Cast",
    casting: "Casting…",
    reset_code: "Reset spell",
    next_puzzle: "Next ▶",
    prev_puzzle: "◀ Prev",
    editor_hint: "Write your spell in real Rust. The world only yields to code that compiles.",
    solved_badge: "GATE OPEN",

    verdict_pass_title: "The gate opens",
    verdict_pass_body: "The runes accept your spell. The forest lets you pass.",
    verdict_compile_title: "The world refuses",
    verdict_compile_body: "The borrow checker speaks:",
    verdict_trial_title: "The trial holds",
    verdict_trial_body: "Your spell compiles, yet the hidden trial is not satisfied:",
    verdict_death_title: "YOU DIED",
    verdict_death_body: "Your spell panicked. The night claims another hunter.",
    verdict_timeout_title: "Lost in the loop",
    verdict_timeout_body: "Your spell wandered past the time the world allows.",

    hint_whisper: "🕯 Ask the lantern",
    hint_exhausted: "The lantern has no more to say.",
    toolbox_label: "🔧 Tools you may need",
    difficulty_label: "Covenant",
    difficulty_normal: "Nightfarer",
    difficulty_easy: "Merciful",
    difficulty_hardcore: "Unforgiven",
    difficulty_hint: "Merciful: the lantern is free — hints cost no echoes.",
    difficulty_effect_normal: "Nightfarer — the standard vigil: hints cost echoes (5 / 10 / 20).",
    difficulty_effect_easy: "Merciful — the lantern is free and fully lit: every hint shown, no echoes spent. For those new to Rust.",
    difficulty_effect_hardcore: "Unforgiven — nothing is saved to disk, and the run's end wipes all progress. The true roguelike covenant.",

    echoes: "echoes",
    stain_here: "Your lost echoes lie here. Pass the trial to reclaim them.",
    stain_away: "Echoes lost at",

    raw_diagnostic: "The raw words of the world",

    combat_miss: "MISS",
    combat_blocked: "BLOCKED",
    combat_lost: "LOST",
    combat_cursed: "CURSED",

    curse_label: "curse",
    wound_line_pre: "the wound is at line ",
    wound_line_post: "",

    sound_mute: "Mute",
    sound_sfx: "Effects",
    sound_bgm: "Music",

    journal_button: "📖 Journal",
    journal_title: "The Hunter's Journal",
    journal_trials: "Trials passed",
    journal_concepts: "Rust mastered",
    hunter_label: "Hunter",
    hunter_default: "Outlander",
    map_title: "The Night Lands",
    map_button: "🗺 Map",
    map_enter: "Enter",
    map_locked_hint: "Sealed — clear more of the region before it.",

    e0382: Voice {
        line: "“You gave that away, hunter. What is given is gone.”",
        note: "A value was moved: after ownership transfers, the old name is dead. Lend it with & instead, or forge a twin with .clone().",
    },
    e0384: Voice {
        line: "“That vow was carved immutable. It will not bend twice.”",
        note: "You assigned twice to an immutable binding. Declare it `let mut` if it was made to change.",
    },
    e0308: Voice {
        line: "“You promised one thing and brought another.”",
        note: "Mismatched types: the expected type and the actual type disagree. Check what the signature (or the other branch) demands.",
    },
    e0369: Voice {
        line: "“Those two were never meant to be joined.”",
        note: "This operator does not work between these types — often words (&str) where a number was needed. Convert or rebind first.",
    },
    e0499: Voice {
        line: "“Two hands on one blade — one of you lets go, or the blade decides.”",
        note: "Two mutable borrows of the same value at once. Only one exclusive grip may exist; end one before taking the other.",
    },
    e0502: Voice {
        line: "“They are still reading, hunter. You do not rewrite a page under a reader's eyes.”",
        note: "A mutable borrow while shared borrows are alive. Let the readers finish (end the & borrows) before you take &mut.",
    },
    e0106: Voice {
        line: "“How long must this promise hold? The world demands you say it.”",
        note: "A reference needs a lifetime the compiler cannot infer. Name one (<'a>) so the borrow's span is spelled out.",
    },
    e0425: Voice {
        line: "“No one of that name lives here.”",
        note: "Unresolved name: it was never declared, or it is spelled differently. Check for typos and missing `let`/`fn`.",
    },
};
