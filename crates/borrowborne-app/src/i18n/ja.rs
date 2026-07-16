use super::Tr;

pub static JA: Tr = Tr {
    language: "言語",
    lives: "残機",
    progress: "進行度",
    deaths_total: "墓標",

    cast: "⚔ 詠唱",
    casting: "詠唱中…",
    reset_code: "呪文をリセット",
    next_puzzle: "次へ ▶",
    prev_puzzle: "◀ 前へ",
    editor_hint: "本物の Rust で呪文を書け。世界はコンパイルできるコードにのみ道を譲る。",
    solved_badge: "門は開いた",

    verdict_pass_title: "門が開く",
    verdict_pass_body: "ルーンは呪文を受け入れた。森が道を譲る。",
    verdict_compile_title: "世界は拒んだ",
    verdict_compile_body: "借用チェッカーが告げる：",
    verdict_trial_title: "試練は破れず",
    verdict_trial_body: "呪文はコンパイルされたが、隠された試練は満たされていない：",
    verdict_death_title: "YOU DIED",
    verdict_death_body: "呪文はパニックした。夜がまた一人の狩人を連れ去る。",
    verdict_timeout_title: "ループの迷い子",
    verdict_timeout_body: "呪文は世界が許す時を越えて彷徨った。",
};
