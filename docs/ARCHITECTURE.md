# Borrowborne Architecture

Three crates, one boundary each, mirroring the gamegene layout:
**core** holds pure logic, **runner** isolates the dangerous edge
(compiling and executing player code), **app** is the egui front end.
Content is data, not code.

## Tree

```
borrowborne/
├── Cargo.toml                      # workspace; version/edition/license via workspace.package
├── README.md                       # vision, gameplay, build steps
├── CHANGELOG.md                    # Keep-a-Changelog
├── ROADMAP.md                      # chapter release order = learning path
├── LICENSE                         # MIT
├── docs/
│   └── ARCHITECTURE.md             # this file
│
├── assets/                         # logo, mascot art, icon tooling
│   └── borrowborne.svg             # hunter's-mark `&` rune (Bloodborne-style homage)
│
├── content/                        # ★ curriculum as data — no code changes to add puzzles
│   └── chapters/                   # one .ron = one Rust chapter = one map region
│       ├── 01-newbie-village.ron   #   variables / mut / control flow
│       ├── 02-ownership-forest.ron #   move / borrow (the borrow checker is physics)
│       ├── 03-enum-town.ron        #   enum / match (missing arm = no compile = stuck)
│       ├── 04-result-swamp.ron     #   Option/Result/?  · panic! = permadeath
│       ├── 05-trait-guild.ron      #   traits / generics (class = impl trait)
│       ├── 06-iter-library.ron     #   Vec/HashMap/iterator puzzles
│       ├── 07-lifetime-shrine.ron  #   lifetimes
│       └── 08-concurrency-keep.ron #   thread/channel/Arc — the final maze
│
└── crates/
    ├── borrowborne-core/           # ── pure logic, zero egui
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── constants.rs        # ★ single identity/tuning point: APP_NAME, timeouts…
    │   │   ├── error.rs            # thiserror
    │   │   ├── curriculum/
    │   │   │   ├── mod.rs          # Curriculum / Chapter / Puzzle types
    │   │   │   ├── loader.rs       # parse content/chapters/*.ron
    │   │   │   └── concept.rs      # Concept enum = skill-tree node
    │   │   ├── verdict.rs          # Passed / CompileError / TrialFailed / Panicked / Timeout
    │   │   └── progress.rs         # save model: solved puzzles, deaths
    │   └── tests/
    │       └── curriculum_tests.rs # every .ron parses; concepts covered
    │
    ├── borrowborne-runner/         # ── the dangerous edge (≙ gamegene-platform)
    │   ├── src/
    │   │   ├── lib.rs              # trait Sandbox { evaluate(code, trial) -> Verdict }
    │   │   ├── harness.rs          # wraps player code + hidden trial into one compilation unit
    │   │   ├── rustc_local.rs      # native: Command → rustc, timeout, stderr → Verdict
    │   │   └── playground.rs       # web: request/verdict protocol for the Rust Playground API
    │   └── tests/
    │       ├── runner_selftest.rs  # good code passes / borrow error blocked / panic caught
    │       └── playground_tests.rs # canned playground responses → verdicts (no network)
    │
    └── borrowborne-app/            # ── egui front end (≙ gamegene-app)
        ├── src/
        │   ├── main.rs
        │   ├── app/
        │   │   ├── mod.rs          # App struct + eframe::App::update
        │   │   ├── chrome.rs       # top bar: title / lang / progress
        │   │   ├── puzzle.rs       # scene panel + code editor + Cast button
        │   │   └── verdict_view.rs # compiler output performed as NPC dialogue
        │   ├── fx/
        │   │   ├── mod.rs          # Fx state; drives request_repaint while animating
        │   │   ├── particles.rs    # pass: particle burst
        │   │   └── shake.rs        # panic: screen shake
        │   ├── i18n/               # compile-time-checked Tr struct (en / zh_hant / ja)
        │   ├── theme.rs            # gothic dark palette
        │   └── fonts.rs            # system CJK fallback
        └── tests/
            └── layout_probe.rs     # headless layout verification
```

## Boundaries and why

- **core is pure.** `Verdict`, `Puzzle`, `Progress` are plain data —
  tests never open a window, and the UI can be swapped (Bevy later)
  without touching game logic. core depends on nothing in the
  workspace.
- **runner is the only crate allowed to touch a compiler or spawn a
  process.** Everything behind `trait Sandbox`, so the MVP
  `RustcLocal` backend (temp cargo dir + `rustc` + run with timeout)
  can be replaced by a `wasm32` + wasmtime sandbox without the app
  noticing. This mirrors how gamegene quarantines OS memory access in
  `gamegene-platform`.
  - **The web build has no local `rustc`**, so `rustc_local` is
    compiled out on `wasm32` and `playground.rs` speaks the official
    Rust Playground's execute API instead. The module stays pure —
    build a request body, judge a response — because native threads
    and browser fetch differ too much to hide behind the sync
    `Sandbox` trait; the app owns the actual HTTP call.
- **app owns presentation only.** It converts `Verdict` into drama:
  stderr becomes NPC dialogue, `Panicked` triggers permadeath +
  screen shake, `Passed` fires particles. i18n uses a `Tr` struct so a
  missing translation is a compile error, not a runtime hole.
- **content is data.** Adding a puzzle or a whole region is a `.ron`
  edit; `curriculum_tests.rs` guards that every file parses and every
  concept has coverage.

## The core loop (data flow)

```
player code (String)
      │  app/puzzle.rs — Cast
      ▼
core::curriculum::Puzzle ── harness.rs ──► one main.rs: player code + hidden trial
      │                                          │
      │                                rustc_local.rs: compile, run, timeout
      ▼                                          ▼
app/verdict_view.rs ◄──────────────── core::Verdict (pure data)
  Passed      → gate opens, particles, progress saved
  CompileError→ the world refuses (NPC reads the diagnostic)
  Panicked    → PERMADEATH, shake, death counter
```

## Effects note (egui)

egui repaints on demand: any running animation must call
`ctx.request_repaint()` every frame or it freezes. `fx::Fx::tick`
centralizes that — it is the only place allowed to request repaints
for animation.

## Security note

The MVP backend executes player-written native code on the local
machine — acceptable for a single-player learning game you run on
yourself, but the wasmtime backend (ROADMAP 0.5+) is the real answer
before ever accepting third-party content.
