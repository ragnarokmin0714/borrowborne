use super::Tr;

pub static EN: Tr = Tr {
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
};
