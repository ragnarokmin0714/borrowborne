# Borrowborne

> *Fear the old blood. Respect the borrow checker.*

**Borrowborne** is a roguelike puzzle RPG where the spells are real Rust
code. Every puzzle is a locked door; the key is a program that compiles.
The borrow checker is not a tutorial pop-up — it is a law of physics in
this world. `panic!` is permadeath.

Finish the game and you have, quite literally, learned Rust: each map
region is one chapter of the language, and its gates only open for code
that the real `rustc` accepts.

## How it plays

- **Read the scene** — an NPC states the problem (left panel).
- **Write the spell** — real Rust, in the in-game editor.
- **Cast** — the code is compiled and run against hidden trials. The
  trial is a monster: a compile error whiffs (the borrow checker
  speaks in character — raw diagnostics stay one click away), a failed
  trial is blocked, and a pass slashes its health bar down.
- **The world answers** — a `panic!` at runtime kills your character.
  Roguelike rules apply.

### Blood echoes

First-solving a puzzle earns **echoes**. Dying drops your whole purse
as a bloodstain at the puzzle that killed you — re-solve it to reclaim
them; die again holding echoes and the old stain is lost forever.
Echoes buy **hints**, three tiers per puzzle: a concept nudge, the
faulty line, a near-solution.

### Run curses

Every run rolls one curse: `.clone()` forbidden, `.unwrap()` forbidden,
or a per-cast echo tax. Curses bend the run's rules, never the judge —
whatever reaches the compiler is judged exactly as ever.

## The map is the curriculum

| Region | Rust concepts |
|---|---|
| Newbie Village | variables, `mut`, control flow |
| Ownership Forest | move semantics, `&` / `&mut` borrows |
| Enum Town | `struct`, `enum`, exhaustive `match` |
| Result Swamp | `Option`, `Result`, `?`, `panic!` = permadeath |
| Trait Guild | traits, generics — your class is an `impl` |
| Iterator Library | `Vec`, `HashMap`, iterator chains |
| Lifetime Shrine | lifetimes |
| Concurrency Keep | threads, channels, `Arc`/`Mutex` — the final maze |

## Playing

**In the browser** — the web build deploys to GitHub Pages on every
push to `main`:

> https://ragnarokmin0714.github.io/borrowborne/

On the web your spells are judged by the official
[Rust Playground](https://play.rust-lang.org)'s execute API (no local
toolchain needed — but it does need the network, and CJK UI strings are
not yet bundled for the web).

**Natively** — requires a Rust toolchain (the game shells out to your
local `rustc` to judge your spells — MVP backend; a wasmtime sandbox is
on the roadmap):

```sh
cargo run --release -p borrowborne-app
```

Or grab a prebuilt binary from
[Releases](https://github.com/ragnarokmin0714/borrowborne/releases) —
tagged `v*` pushes build Linux and Windows archives automatically.

To build the web bundle locally:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
cd crates/borrowborne-app && trunk serve
```

## Workspace layout

Three crates, one boundary each — see
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the full tree and the
reasoning:

- `borrowborne-core` — pure game/curriculum logic, zero UI.
- `borrowborne-runner` — the dangerous edge: compiles and runs player code.
- `borrowborne-app` — the egui front end (map, editor, effects, i18n).

Puzzles are data, not code: `content/chapters/*.ron`.

## License

MIT.
