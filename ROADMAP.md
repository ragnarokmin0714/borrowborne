# Borrowborne Roadmap

The release order *is* the learning path: each milestone ships one map
region = one chapter of Rust. A region ships when its puzzles cover the
chapter's concepts and the layout probe + runner selftests pass.

Sequencing principle: **0.2 fixes "too hard so I quit"; 0.3–0.4 fix
"not juicy enough"** — retention before spectacle.

## Save policy (deliberate, not an oversight)

Progress persists from day one (eframe persistence: OS config dir on
native, localStorage on web). Roguelike death resets the *run's lives*;
solved gates and learned concepts survive — knowledge does not die with
the hunter. Crash safety beats purity while the game is young. A later
**hardcore difficulty** opts back into no-save/full-reset for players
who want the true roguelike covenant.

## 0.1 — Core loop (done)

- [x] Workspace, docs, CI conventions.
- [x] Write → compile → trial → gate-opens loop against local `rustc`.
- [x] Ownership Forest (move / `&` / `&mut`), 3 puzzles.
- [x] Web build on GitHub Pages (Rust Playground as the judge).
- [x] Release pipeline: `v*` tags publish Linux/Windows archives.
- [x] Progress persistence (solved gates + learned concepts survive
      restarts and crashes).

## 0.2 — Learnable: content ×3, errors as drama, ladders

- [ ] Newbie Village: variables, `mut`, `if`/`loop`/`while` (6–8 puzzles).
- [ ] Ownership Forest expansion: `clone` costs, `&str` vs `String` (2–3).
- [ ] E-code → NPC dialogue: E0382 / E0502 / E0499 / E0308 / E0106 get
      in-world lines + plain-language explanations; raw stderr stays,
      collapsed.
- [ ] Hint system: 3 tiers per puzzle (concept nudge → faulty line →
      near-solution), paid with a small resource cost. `hints: []` field
      in the `.ron` schema.
- [x] Save-on-verdict: persist immediately after each pass/death instead
      of only on eframe's autosave tick.

## 0.3 — Addictive: blood echoes, combat skin, curses

- [x] Blood echoes: currency dropped on pass; **your echoes drop where
      you die** — re-solve the puzzle to reclaim them (the corpse run is
      "go face your bug"). Hints are the first echo sink (5/10/20).
- [x] Combat presentation over the same verdicts: monster HP bar,
      compile error = whiff (MISS), trial failed = blocked, timeout =
      lost, pass = kill (slash + HP drain + echo gain float), panic =
      counterattack (death flash + dropped-echo float). Presentation
      only — the judge stays the compiler.
- [ ] Learned concepts unlock skill animations.
- [x] Curse modifiers per run (data-driven, `content/curses.ron`):
      forbid-snippet curses (no `.clone()`, no `.unwrap()`) and a
      per-cast echo tax; rerolled when the run ends.
- [ ] More curse effects: randomized trial inputs, heavier deaths.
- [x] Map screen: region nodes, seals break at 70% of the previous
      region; the game opens on the map.
- [x] Enum Town: `struct` / `enum` / exhaustive `match` (6 puzzles).
- [x] Result Swamp: `Option` / `Result` / `?` (6 puzzles).

## 0.4 — Audible & measurable: sound, performance trials

- [x] SFX first (kira: native + WebAudio): seven synthesized sounds,
      one per dramatic beat; lazy device init doubles as the browser
      autoplay gesture; mute toggle persisted.
- [x] BGM second: synthesized seamless drone per region, crossfading
      on travel; no assets, no licensing.
- [x] Performance trials engine: the harness times each trial (one
      stopwatch for both judges); S/A/B grades pay echo bonuses and
      persist per puzzle. TLE armor is a content pattern (huge inputs
      vs. the 5 s budget) — first armored puzzles ship with the
      Iterator Library below.
- [x] Trait Guild: pick a class by implementing its trait — impls,
      default methods, bounds, derives, `dyn`, and a Guildmaster boss.
- [x] Iterator Library: iterator-chain puzzles over `Vec` / `HashMap`,
      carrying the first two TLE-armored doors (correct-but-slow
      starters that die on the 5 s budget).

## 0.5 — Endgame regions

- [ ] Lifetime Shrine: lifetime annotation puzzles.
- [ ] Concurrency Keep: threads / channels / `Arc<Mutex>` final maze.
- [ ] Difficulty tiers, including **hardcore**: no save, one run,
      LeetCode-style algorithm dungeon lives here.

## Later

- [ ] wasmtime sandbox backend (`runner::wasm`) — required before any
      third-party chapter content.
- [ ] Hidden region: macros / `unsafe`.
- [ ] Skill-tree journal view; achievements; run statistics.
- [x] Subset a CJK font for the web build (zh-Hant/ja chrome glyphs) —
      ~104 KiB embedded, coverage-tested.
- [ ] Steam-deckable packaging.
