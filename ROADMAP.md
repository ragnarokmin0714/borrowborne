# Borrowborne Roadmap

The release order *is* the learning path: each milestone ships one map
region = one chapter of Rust. A region ships when its puzzles cover the
chapter's concepts and the layout probe + runner selftests pass.

## 0.1 — Core loop (done)

- [x] Workspace, docs, CI conventions.
- [x] Write → compile → trial → gate-opens loop against local `rustc`.
- [x] Ownership Forest (move / `&` / `&mut`), 3 puzzles.

## 0.2 — Newbie Village + polish

- [ ] Newbie Village: variables, `mut`, control flow (5+ puzzles).
- [ ] Friendly-fied compiler errors: map common E-codes to NPC dialogue.
- [ ] Map screen: region selection with lock/unlock states.

## 0.3 — Enum Town & Result Swamp

- [ ] Enum Town: `struct` / `enum` / exhaustive `match`.
- [ ] Result Swamp: `Option` / `Result` / `?`; permadeath economy
      (deaths cost progress, roguelike seed reshuffles encounters).

## 0.4 — Trait Guild & Iterator Library

- [ ] Trait Guild: pick a class by implementing its trait.
- [ ] Iterator Library: iterator-chain puzzles over `Vec` / `HashMap`.

## 0.5 — Lifetime Shrine & Concurrency Keep

- [ ] Lifetime Shrine: lifetime annotation puzzles.
- [ ] Concurrency Keep: threads / channels / `Arc<Mutex>` final maze.

## Later

- [ ] wasmtime sandbox backend (`runner::wasm`) replacing raw local exec.
- [ ] Hidden region: macros / `unsafe`.
- [ ] Skill-tree journal view; achievements; run statistics.
- [ ] Steam-deckable packaging.
