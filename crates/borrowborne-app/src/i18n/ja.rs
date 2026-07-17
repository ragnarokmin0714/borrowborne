use super::{Tr, Voice};

pub static JA: Tr = Tr {
    tagline: "古き血を恐れよ。借用チェッカーを敬え。",
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

    hint_whisper: "🕯 ランタンに問う",
    hint_exhausted: "ランタンはもう何も語らない。",

    echoes: "残響",
    stain_here: "失った残響はここに眠る。試練を越えて取り戻せ。",
    stain_away: "残響を失った場所：",

    raw_diagnostic: "世界の生の言葉",

    combat_miss: "空振り",
    combat_blocked: "防がれた",
    combat_lost: "迷失",
    combat_cursed: "呪縛",

    curse_label: "呪い",

    sound_mute: "ミュート",
    sound_sfx: "効果音",
    sound_bgm: "音楽",

    hunter_label: "狩人",
    hunter_default: "異邦人",
    map_title: "夜の国",
    map_button: "🗺 地図",
    map_enter: "入る",
    map_locked_hint: "封印中——前の地域をさらに踏破せよ。",

    e0382: Voice {
        line: "「渡したものだ、狩人よ。手放したものは戻らぬ。」",
        note: "値はムーブされた：所有権が移れば、古い名は死ぬ。& で貸すか、.clone() で双子を鍛えよ。",
    },
    e0384: Voice {
        line: "「その誓いは不変と刻まれた。二度は曲がらぬ。」",
        note: "不変の束縛に二度代入した。変わるべきものなら `let mut` と宣言せよ。",
    },
    e0308: Voice {
        line: "「約束したものと、持ってきたものが違う。」",
        note: "型の不一致：期待される型と実際の型が食い違っている。シグネチャ（または他の分岐）の要求を確かめよ。",
    },
    e0369: Voice {
        line: "「その二つは、結ばれる定めにない。」",
        note: "この演算子はこれらの型の間では使えない——数のあるべき所に文字（&str）があることが多い。先に変換か再束縛を。",
    },
    e0499: Voice {
        line: "「一振りの刃に両の手——どちらかが放せ。さもなくば刃が決める。」",
        note: "同じ値への可変借用が二つ同時に存在する。独占の握りは一つだけ——一方を終えてから次を取れ。",
    },
    e0502: Voice {
        line: "「まだ読んでいる者がいる。読者の眼前で頁を書き換えるな。」",
        note: "共有借用（&）が生きている間に可変借用（&mut）を取った。読者を先に帰し、それから &mut を。",
    },
    e0106: Voice {
        line: "「その約束、いつまで守る？　世界は明言を求める。」",
        note: "コンパイラが推論できないライフタイムがある。<'a> と名付け、借用の寿命を明記せよ。",
    },
    e0425: Voice {
        line: "「その名の者は、ここには住んでおらぬ。」",
        note: "名前が解決できない：宣言されていないか、綴りが違う。誤字と `let`/`fn` の抜けを確かめよ。",
    },
};
