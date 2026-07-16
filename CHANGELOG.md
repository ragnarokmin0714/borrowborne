# Changelog

All notable changes to Borrowborne are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/); versions follow SemVer.

## [Unreleased]

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
