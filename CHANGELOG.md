# Changelog

All notable changes to Borrowborne are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/); versions follow SemVer.

## [Unreleased]

### Added

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
