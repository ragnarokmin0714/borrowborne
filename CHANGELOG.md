# Changelog

All notable changes to Borrowborne are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/); versions follow SemVer.

## [Unreleased]

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
