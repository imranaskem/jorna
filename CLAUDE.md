# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Jorna is a terminal-based HTTP client built with Rust and Ratatui. It provides a keyboard-driven TUI for making HTTP requests with session persistence.

## Commands

```bash
cargo build --release          # Production build
cargo run                      # Development run
cargo test                     # All tests
cargo test app::tests          # App module tests only
cargo test <test_name>         # Single test by name
cargo fmt --all -- --check     # Check formatting
cargo clippy --all-targets --all-features -- -D warnings  # Lint
```

CI runs `fmt --check`, `clippy -D warnings`, `cargo test`, and a release build.

## Architecture

**Data Flow:** State (app/) → Events (event/) → Rendering (ui/)

- `src/main.rs` — Terminal setup, event loop, panic cleanup
- `src/app/mod.rs` — `App` struct (all application state), HTTP request logic, text editing helpers
- `src/event/mod.rs` — Keyboard event routing, dispatches to `App` methods based on `AppFocus`
- `src/ui/mod.rs` — Ratatui rendering
- Each module has a colocated `tests.rs` file

### Key Design Patterns

- **Single state struct:** `App` holds everything — URL, headers, body, response, cursor positions, scroll offsets, focus state
- **Focus-driven input:** `AppFocus` enum (`MethodSelector`, `UrlInput`, `HeadersInput`, `BodyInput`, `Response`) determines which key handler runs
- **Multi-line text fields:** Headers and body use `Vec<String>` with separate `cursor_line`/`cursor_col`/`scroll` tracking (parameterized via `is_headers: bool`)
- **Session persistence:** State serialized to `~/.jorna/state.json` via serde; saved after every key event, restored on startup
- **Blocking HTTP:** Uses `reqwest::blocking` — the UI freezes during requests (the `loading` flag prevents input)
- **Testing:** Uses Ratatui's `TestBackend` for UI tests

### Adding a Feature

1. Add state fields to `App` struct in `src/app/mod.rs`
2. Add keyboard handling in `src/event/mod.rs`
3. Add UI rendering in `src/ui/mod.rs`
4. Write tests in the corresponding `tests.rs` files
