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
- [ ] Curse modifiers per run (data-driven): "no `.clone()` this run",
      "unwrap kills instantly", randomized trial inputs.
- [ ] Map screen: region nodes, lock/unlock states.
- [ ] Enum Town: `struct` / `enum` / exhaustive `match`.
- [ ] Result Swamp: `Option` / `Result` / `?`.

## 0.4 — Audible & measurable: sound, performance trials

- [ ] SFX first (kira: native + WebAudio): cast, hit, gate, YOU DIED.
- [ ] BGM second: looped region themes, CC0-licensed assets.
- [ ] Performance trials (honest LeetCode-style TLE, not fake Big-O
      detection): late puzzles feed n=10⁵ inputs under the existing
      timeout — O(n²) bounces off "the monster's armor"; finish-time
      grades S/A/B multiply damage/echoes.
- [ ] Trait Guild: pick a class by implementing its trait.
- [ ] Iterator Library: iterator-chain puzzles over `Vec` / `HashMap`.

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
- [ ] Subset a CJK font for the web build (zh-Hant/ja chrome glyphs).
- [ ] Steam-deckable packaging.
