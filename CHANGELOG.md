# Changelog

All notable changes to Borrowborne are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/); versions follow SemVer.

## [Unreleased]

### Changed

- **One settings menu (⚙)**: language, text size, sound, and the
  version now live together behind a single gear in the top bar,
  instead of scattered controls that crowded each other (the A-/A+
  buttons in particular collided with their label). The top bar keeps
  only navigation on the left and run stats on the right; a mute
  indicator rides the gear (⚙ 🔇) so sound state still reads at a glance.

## [0.8.0] - 2026-07-21

### Changed (playability feedback)

- **Adjustable text size (UI zoom)**: an A-/A+ control in the top bar
  scales the whole interface — the code editor most of all — for big
  and high-DPI screens where the fixed sizes were still too small.
  Persisted, and keyboard zoom (Ctrl +/-) sticks too. The base
  monospace size also went up again (16 → 18 px).

### Added

- **The Hunter's Journal**: a new screen (📖 in the top bar) that makes
  the whole point of the game visible — the Rust you've learned. It
  shows trials passed, concepts mastered, deaths and echoes, and a
  per-region skill tree where each concept lights gold once a puzzle
  teaches it. Concept labels stay in Rust's own terms (`&mut`, `match`,
  `Arc<Mutex>`) in every language; the hardcore dungeon's nodes stay
  hidden off the covenant, here as on the map.
- **Algorithm Dungeon** (5 puzzles, hardcore-only): a vault of
  complexity that appears on the map only under the Unforgiven
  covenant, its node drawn in blood. Two-sum, sliding window, in-place
  reversal, prefix sums, and a Kadane max-subarray boss. Four are
  TLE-armored — the starter is a *correct* O(n²) or O(n·k) that the
  trial floods (100k–200k inputs) until it blows past the 5 s budget;
  the solution is the same answer in linear time. Regions can now be
  flagged `hardcore_only` in the `.ron` schema. 54 puzzles, 9 regions
  total, with its own BGM drone.
- **Unforgiven covenant (hardcore)**: the third difficulty, the true
  roguelike vow — progress is never written to disk, and the run's end
  (seven deaths) wipes everything: solved gates, echoes, grades, all of
  it. Every launch is a fresh gauntlet; a Normal/Easy save on disk is
  left untouched underneath, so switching back restores it. Difficulty
  is now a persisted *setting* rather than part of progress, which is
  what lets the no-save covenant stay clean.

## [0.7.0] - 2026-07-18

### Changed (playability feedback)

- **Bigger fonts**: every text style is larger now, the monospace spell
  editor most of all (egui's ~12 px default → 16 px) — the code was
  the most-read and smallest text in the game.
- **Merciful now lights the whole lantern**: in the Merciful covenant
  every hint tier is shown at once (still free), so a beginner is never
  stranded one paid whisper short of the near-solution line. This
  answers the worry that even Easy could be "quit on entry" for someone
  who has never written Rust. Nightfarer still reveals one paid tier at
  a time.

### Fixed

- **Editor highlighting split identifiers with an embedded keyword**:
  `true_age` rendered as `true` (red keyword) + `age` (green) because
  the highlighter breaks words at underscores. Renamed to `real_age`;
  a scan confirms it was the only such identifier in the content.

### Added

- **Concurrency Keep** (6 puzzles): the final maze. `thread::spawn`
  and `join`, the `move` closure, an `mpsc` channel, `Arc` for a
  shared read, `Arc<Mutex>` for a shared write, and a five-shift boss
  totalling one guarded ledger. Every trial joins all threads before
  it reads, so the puzzles are deterministic despite being threaded —
  no timing races, well under the 5 s budget. (cc-05/cc-06's canonical
  solutions bind the lock read to a local so the `MutexGuard` drops
  before the `Arc`, avoiding E0597 — a real Rust gotcha the content
  gate caught.) 49 puzzles, 8 regions total, with its own BGM drone.
- **Lifetime Shrine** (5 puzzles): the first endgame region. The
  function lifetime (`longest`-style `<'a>`), a struct that borrows,
  the lifetime carried onto an `impl` block, the dangling-borrow wall
  that no annotation can save (return owned instead), and a
  keeper-of-graves boss tying struct + impl + a method whose returned
  reference is bound to `'a`. Starters fail with the same E0106 /
  E0726 / E0515 family the content gate compiles for real. 43 puzzles,
  7 regions total, with its own BGM drone.
- **Toolbox nudges**: every puzzle now shows a free, always-visible
  list of the syntax, methods, and types it may call for (e.g.
  `.unwrap_or()`, `match`, `Box<dyn Trait>`) — a step short of a hint,
  naming the tools without saying how to use them. It costs nothing
  and never competes with the paid lantern hints. Authored for all 38
  puzzles; a new optional `toolbox` field in the `.ron` schema.
- **Merciful covenant (Easy difficulty)**: chosen on the world map,
  it makes the lantern free — hints cost no echoes — for players who
  find the borrow checker punishment enough. Persisted with progress;
  Nightfarer (Normal) keeps the echo economy. A standing note under
  the picker spells out what the chosen covenant actually does, so the
  difference is never a mystery.
- **BGM melody**: each region's drone now carries a sparse plucked
  melody — a minor-pentatonic motif riding above the root, rotated per
  region so no two regions play the same tune. The pluck envelopes
  decay to silence before the 12 s loop point, so the seam stays
  clickless (unit-tested, along with each region being distinct). This
  answers "the music is too monotonous": the drone was a flat pad.

## [0.6.0] - 2026-07-18

### Added

- **Editor intelligence, first tier**: the spell editor now syntax-
  highlights Rust (egui's built-in highlighter — no heavyweight
  dependency, works on the web build too), and compile errors /
  panics point at the wound: the verdict names the offending line of
  the player's own code and quotes it, in all three languages, instead
  of leaving the line number buried in raw stderr. Locations that fall
  in harness or trial territory are never mapped — a wrong line would
  be worse than none.

- **Trait Guild** (6 puzzles): keep a trait's oath (`impl`), override
  a default method, bound generics, `#[derive]` the common powers,
  box a mixed brigade behind `dyn`, and a Guildmaster boss demanding
  an own impl plus a bounded generic duel.
- **Iterator Library** (7 puzzles): `map`/`collect`, `filter`/`sum`,
  the HashMap `entry` API, `max_by_key`, and a `filter→map→sum` boss.
  Includes the first two **TLE-armored doors**: correct-but-quadratic
  duplicate search against 100 000 cards, and an exploding recursion
  against a 45-step stair — both die on the 5-second budget until
  rewritten (HashSet membership; iterative climb). 38 puzzles, 6
  regions total; both new regions carry their own BGM drones.
- **The Outlander** (異鄉人 / 異邦人): the never-named hunter is now
  the localized "Outlander" instead of the English-only "Good
  Hunter". The save keeps an empty name until the player chooses one
  (so switching language switches the default too); old saves still
  holding "Good Hunter" migrate back to nameless.

- **Speed grades (S/A/B)**: the harness now times the trial itself
  and prints the elapsed millis with the pass marker — local `rustc`
  and the Playground share one stopwatch that excludes compile time
  and network. S (≤50 ms) pays +25 echoes on first solve, A (≤500 ms)
  +10; the best grade per puzzle is kept in the save and a ⚡S/⚡A
  float celebrates fast kills. This is the honest LeetCode model:
  measured wall time under the trial's workload, no fake Big-O
  detection — TLE armor (huge inputs vs. the 5 s budget) arrives
  with the Iterator Library content.
- **Volume controls**: the speaker button is now a menu — mute plus
  independent Effects and Music sliders (kira sub-tracks), persisted.
- **BGM audibility**: drones gained ×3/×4/×6 mid partials — laptop
  and phone speakers cannot reproduce the 37–62 Hz roots at all, so
  without the mids the music was physically inaudible on them.
- **BGM**: one seamless ambient drone per region (plus the map),
  synthesized like the SFX — root/fifth/octave layers breathing on
  slow LFOs, every partial an integer number of cycles per 12-second
  loop so the seam is mathematically silent. Crossfades on region
  change; the first click anywhere opens the audio device (the
  browser's autoplay gesture); mute stops it.
- **Sound effects** (kira + cpal): seven procedurally synthesized
  sounds — cast whoosh, kill slash+chime, miss thud, blocked clank,
  cursed tritone, death boom, timeout wobble. No audio assets: every
  sound is math rendered at init. The device opens lazily on the
  first cast, which doubles as the user gesture browsers require.
  Mute toggle (🔊/🔇) in the top bar, persisted. Linux builds now
  need `libasound2-dev` + `pkg-config` (README + CI updated).
- **Hunter name** (issue #1 suggestion): every hunter starts as
  "Good Hunter" and can be renamed on the world map. Persisted with
  progress; empty names fall back, long names clip at 24 chars.

### Fixed (issue #1 feedback)

- **Boxes (□) instead of symbols on the web**: the subset font now
  carries every non-ASCII glyph the sources use (│ ▼ ◉ were only in
  the monospace default); the tombstone marker and curse icon were
  characters no shipped font has and are now ✝ and 🌑. The coverage
  test now scans *all* app sources against the union of shipped
  fonts, so an uncovered character fails CI.
- **Web favicon**: the hunter's mark now marks the browser tab.
- **Untranslated tagline**: "Fear the old blood…" moved into the
  i18n table (zh-Hant / ja translated). Puzzle content itself remains
  authored in English by design — content localization is a separate
  roadmap item.

## [0.5.0] - 2026-07-17

### Added

- **World map**: the game now opens on "The Night Lands" — one node
  per region joined by a night road, each showing its progress.
  Entering a region lands on its first unsolved door. Later regions
  are sealed until 70% of the region before them is solved; prev/next
  navigation respects the seals.
- **Enum Town** (6 puzzles): structs and field completion, `impl`
  methods, exhaustive `match`, data-carrying variants, the lazy-
  wildcard trap, and a stall-keeper boss (`saturating_sub` underflow).
- **Result Swamp** (6 puzzles): `Option` handling (an invited unwrap
  death opens the region), `match` on `Option`, `Result` answering,
  `?` on `Result` and on `Option`, and the Swamp Heart boss
  (`map_err` + `?` + guard). 25 puzzles, 4 regions total.
- **CJK on the web**: a ~104 KiB subset of Noto Sans CJK TC (SIL OFL)
  is embedded as the universal last font fallback — Traditional
  Chinese and Japanese chrome now renders in the browser. The subset
  carries every glyph the i18n strings use plus the kana and
  fullwidth ranges; `assets/make_cjk_subset.py` regenerates it and a
  coverage test fails if any i18n character lacks a glyph. Native
  builds still prefer a full system CJK font when present.

## [0.4.0] - 2026-07-16

### Added

- **Blood echoes**: first-solving a puzzle pays 25 echoes; hunters
  start with 30. On death the whole purse drops as a bloodstain at
  the puzzle where you fell — pass that trial to reclaim it. Dying
  again while holding echoes replaces the old stain (those echoes are
  gone); dying empty-handed spares it. Content edits never steal: a
  stain on a removed puzzle refunds on load.
- **Hints now cost echoes** (5 / 10 / 20 per tier); the lantern
  refuses the broke. The purse and any distant stain show in the top
  bar; a stain at the current door is called out in the scene.
- **Combat skin**: the trial is a monster with a health bar above the
  editor. Compile error floats MISS, failed trial BLOCKED, timeout
  LOST; a pass slashes the bar, drains it, and floats the echo gain;
  a panic floats the echoes you dropped. Presentation only — every
  outcome is still decided solely by the compiler.
- **Run curses** (`content/curses.ron`, data-driven): every run rolls
  one modifier — Curse of the Twinless (no `.clone()`), Curse of
  Certainty (no `.unwrap()`), Curse of Poverty (2-echo tax per cast).
  A refused cast floats CURSED and never reaches the judge; whatever
  compiles is judged exactly as ever. The run's curse shows in the
  top bar (hover for its rules) and rerolls when the run ends.

## [0.3.0] - 2026-07-16

### Added

- **Newbie Village**: 8 puzzles (variables, `mut`, shadowing, `if` as
  expression, `loop`/`break`, `while`, integer types, `const`, and a
  combined elder's trial). Puzzle 4 deliberately walks the player into
  the Timeout verdict, puzzle 5 into TrialFailed — every verdict kind
  is met in safety before the night begins.
- **Ownership Forest expansion**: `.clone()` as the honest answer
  (of-04), `&str` vs `String` with deref coercion (of-05). 13 puzzles
  total.
- **Compiler errors as NPC dialogue**: eight common E-codes get an
  in-world line plus a plain-language note (en / zh-Hant / ja); the
  raw diagnostic stays available, collapsed.
- **Tiered hints**: three per puzzle (concept nudge → faulty line →
  near-solution), revealed one at a time; hints re-seal on puzzle
  change and restart.
- **Content gate test**: every puzzle's starter and canonical solution
  are compiled by the real `rustc` in CI — starters must fail,
  solutions must pass. `.ron` edits are now as protected as code.
- Cross-chapter navigation (the last door of a region opens into the
  next); the web build now embeds all chapters (it previously fell
  back to the forest only).
- Save-on-verdict: progress flushes to disk immediately after each
  pass/death instead of waiting for eframe's autosave tick.
- Save hardening: `learned` concepts are no longer serialized (derived
  from solved puzzles on load), stale puzzle ids from older content are
  dropped, and death counters are clamped — a corrupt or outdated save
  self-heals instead of poisoning a run.

## [0.2.0] - 2026-07-16

Note: the `v0.2.0` tag was cut while the workspace still said `0.1.0`,
so its binaries report v0.1.0 in the window title. Release checklist
since then: **bump `workspace.package.version` before tagging.**

### Added

- Web build: the app compiles to `wasm32-unknown-unknown` and runs in
  the browser (trunk + eframe `WebRunner`); spells are judged by the
  official Rust Playground's execute API since no local `rustc` exists
  on the web. `runner::playground` holds the pure request/verdict
  protocol with canned-response tests.
- GitHub Pages deploy workflow (`pages.yml`): every push to `main`
  publishes the playable web build.
- Release workflow (`release.yml`): pushing a `v*` tag builds Linux and
  Windows archives (binary + `content/` + docs) and publishes a GitHub
  Release.

### Known limitations

- Web build ships without a CJK font: Traditional Chinese / Japanese
  chrome shows placeholder glyphs in the browser (accepted for now).

## [0.1.0] - 2026-07-16

### Added

- Cargo workspace: `borrowborne-core` (pure logic),
  `borrowborne-runner` (compiles/runs player code via local `rustc`),
  `borrowborne-app` (egui front end).
- Core loop: write Rust in-game → compile → hidden trials → gate opens.
- Verdict model: `Passed` / `CompileError` / `TrialFailed` / `Panicked`
  (permadeath) / `Timeout`.
- Data-driven curriculum: chapters and puzzles load from
  `content/chapters/*.ron`; first chapter **Ownership Forest** with
  three puzzles (move semantics, shared borrow, mutable borrow).
- Compile-time-checked i18n (`Tr` struct): English, Traditional
  Chinese, Japanese.
- Gothic dark theme; success particle burst and panic screen-shake
  effects.
- Progress tracking (solved puzzles, death count) persisted via eframe.
