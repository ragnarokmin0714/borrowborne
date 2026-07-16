use super::Tr;

pub static ZH: Tr = Tr {
    language: "語言",
    lives: "生命",
    progress: "進度",
    deaths_total: "墓碑",

    cast: "⚔ 施展",
    casting: "詠唱中…",
    reset_code: "重置咒文",
    next_puzzle: "下一關 ▶",
    prev_puzzle: "◀ 上一關",
    editor_hint: "以真正的 Rust 書寫你的咒文。世界只為能編譯的程式碼讓路。",
    solved_badge: "門已開啟",

    verdict_pass_title: "大門開啟",
    verdict_pass_body: "符文接納了你的咒文。森林為你讓出道路。",
    verdict_compile_title: "世界拒絕了你",
    verdict_compile_body: "借用檢查器開口了：",
    verdict_trial_title: "試煉未破",
    verdict_trial_body: "咒文成功編譯，但隱藏的試煉仍未滿足：",
    verdict_death_title: "你死了",
    verdict_death_body: "你的咒文 panic 了。長夜又帶走一位獵人。",
    verdict_timeout_title: "迷失於迴圈",
    verdict_timeout_body: "你的咒文遊蕩超過了世界允許的時間。",
};
