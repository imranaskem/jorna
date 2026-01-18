# Jorna - Claude Context

Jorna is a terminal-based HTTP client built with Rust and Ratatui. It provides a keyboard-driven TUI for making HTTP requests.

## Quick Reference

- **Language:** Rust (Edition 2021)
- **UI Framework:** Ratatui 0.30 with crossterm
- **HTTP Client:** reqwest (blocking API)
- **Version:** 0.1.2
- **License:** MIT

## Architecture

```
src/
├── main.rs          # Entry point, terminal setup, event loop
├── app/
│   ├── mod.rs       # App state, HTTP logic, text editing
│   └── tests.rs     # 50+ unit tests
├── event/
│   ├── mod.rs       # Keyboard event routing
│   └── tests.rs     # 40+ tests
└── ui/
    ├── mod.rs       # Ratatui rendering
    └── tests.rs     # 30+ tests
```

**Data Flow:** State (app/) → Events (event/) → Rendering (ui/)

## Key Concepts

### App State (`src/app/mod.rs`)
- Single `App` struct holds all application state
- `AppFocus` enum tracks which component has focus: `MethodSelector`, `UrlInput`, `HeadersInput`, `BodyInput`, `Response`
- Multi-line text fields (headers, body) use `Vec<String>` with separate cursor tracking
- `loading` flag blocks input during HTTP requests

### Supported HTTP Methods
GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS (defined in `METHODS` array)

### Keybindings
- `Esc`: Quit
- `Tab`: Cycle focus
- `Enter`: Send request (from URL/method/headers)
- `Ctrl+Enter` or `Alt+Enter`: Send from body input
- `Shift+Enter`: New line in multi-line inputs
- Arrow keys: Cursor movement / response scrolling

## Common Tasks

### Adding a new feature
1. Add state fields to `App` struct in `src/app/mod.rs`
2. Add keyboard handling in `src/event/mod.rs`
3. Add UI rendering in `src/ui/mod.rs`
4. Write tests in corresponding `tests.rs` files

### Running tests
```bash
cargo test              # All tests
cargo test app::tests   # App module only
```

### Building
```bash
cargo build --release   # Production build
cargo run               # Development run
```

## CI/CD

- **CI:** Runs on every push - linting (rustfmt, clippy), testing, release build
- **Release:** Triggered after CI on main - builds macOS ARM64 binary, creates GitHub release, updates Homebrew formula

## Dependencies

| Crate | Purpose |
|-------|---------|
| ratatui | Terminal UI framework |
| crossterm | Terminal manipulation |
| reqwest | HTTP client |
| anyhow | Error handling |
| serde_json | JSON parsing |

## Important Patterns

- **Focus-driven input:** Each `AppFocus` state has independent key handlers
- **Validation before send:** Empty URL and invalid JSON are caught before requests
- **Error display:** Network/parsing errors shown in response area
- **Testing:** Uses Ratatui's `TestBackend` for UI tests; colocated test files
