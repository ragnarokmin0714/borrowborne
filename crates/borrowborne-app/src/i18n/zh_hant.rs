use super::{Tr, Voice};

pub static ZH: Tr = Tr {
    tagline: "畏懼古血，敬畏借用檢查器。",
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

    hint_whisper: "🕯 向提燈求問",
    hint_exhausted: "提燈已無話可說。",
    toolbox_label: "🔧 可能會用到的工具",
    difficulty_label: "誓約",
    difficulty_normal: "夜行者",
    difficulty_easy: "慈悲",
    difficulty_hint: "慈悲：提燈免費——提示不花費任何血之迴響。",
    difficulty_effect_normal: "夜行者——標準的守夜：提示需花費迴響（5 / 10 / 20）。",
    difficulty_effect_easy: "慈悲——提燈免費且全亮：所有提示一次顯示，不花費迴響。適合初學 Rust 者。",

    echoes: "迴響",
    stain_here: "你失落的迴響就在此處。通過試煉，將它們取回。",
    stain_away: "迴響失落於",

    raw_diagnostic: "世界的原話",

    combat_miss: "揮空",
    combat_blocked: "被格擋",
    combat_lost: "迷失",
    combat_cursed: "受詛",

    curse_label: "詛咒",
    wound_line_pre: "傷口在第 ",
    wound_line_post: " 行",

    sound_mute: "靜音",
    sound_sfx: "音效",
    sound_bgm: "音樂",

    hunter_label: "獵人",
    hunter_default: "異鄉人",
    map_title: "長夜之地",
    map_button: "🗺 地圖",
    map_enter: "進入",
    map_locked_hint: "封印中——先攻克前一個區域。",

    e0382: Voice {
        line: "「你已把它交出去了，獵人。給出去的，就不再回來。」",
        note: "值已被移動：所有權轉移後，舊的名字就死了。改用 & 借出，或用 .clone() 鑄一個雙生子。",
    },
    e0384: Voice {
        line: "「那誓言刻下時就是不變的。它不會屈折第二次。」",
        note: "你對不可變綁定賦值了兩次。若它生來就該改變，宣告成 `let mut`。",
    },
    e0308: Voice {
        line: "「你承諾的是一物，帶來的卻是另一物。」",
        note: "型別不符：期望的型別與實際的型別不一致。檢查函式簽名（或另一個分支）要求的是什麼。",
    },
    e0369: Voice {
        line: "「這兩者本就無法相連。」",
        note: "這個運算子不能用在這兩種型別之間——常見於該用數字之處拿了文字（&str）。先轉換或重新綁定。",
    },
    e0499: Voice {
        line: "「一柄刀上兩隻手——一人放手，否則刀來決定。」",
        note: "同一個值同時存在兩個可變借用。獨佔的握持只能有一個；先結束其一，再取另一個。",
    },
    e0502: Voice {
        line: "「他們還在讀，獵人。讀者眼前，不得改寫書頁。」",
        note: "共享借用（&）尚存時取了可變借用（&mut）。讓讀者先讀完——結束 & 借用，再取 &mut。",
    },
    e0106: Voice {
        line: "「這承諾要守多久？世界要你親口說出。」",
        note: "有一個引用的生命週期編譯器推不出來。標註一個（<'a>），把借用的存續期明說。",
    },
    e0425: Voice {
        line: "「此地無人叫這個名字。」",
        note: "找不到這個名字：從未宣告過，或拼法不同。檢查錯字，以及是否漏了 `let`/`fn`。",
    },
};
