# Jorna

A terminal-based HTTP client built with Rust and Ratatui. Jorna provides a simple and intuitive TUI for making HTTP requests and viewing responses.

## Features

- **Multiple HTTP Methods**: Support for GET, POST, PUT, DELETE, PATCH, HEAD, and OPTIONS
- **Method Selector**: Easy-to-use method selector with keyboard navigation
- **URL Input**: Full cursor support with text editing capabilities
- **Response Viewer**: Scrollable response viewer with automatic JSON formatting
- **Keyboard-Driven**: Complete keyboard navigation for efficient workflow
- **Real-time Status**: Live status codes and response headers

## Installation

### Homebrew (macOS)

```bash
brew tap imranaskem/jorna
brew install jorna
```

### Building from Source

#### Prerequisites

- Rust 1.70 or higher
- Cargo

#### Build Steps

```bash
git clone <repository-url>
cd jorna
cargo build --release
```

The compiled binary will be available at `target/release/jorna`.

## Usage

Run the application:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/jorna
```

### Keyboard Shortcuts

#### Global

- **Tab**: Cycle focus between Method Selector → URL Input → Response Viewer
- **Esc**: Quit application

#### Method Selector (when focused)

- **↑/↓**: Cycle through HTTP methods
- **Enter**: Send request with selected method

#### URL Input (when focused)

- **Enter**: Send HTTP request
- **←/→**: Move cursor left/right
- **Home**: Jump to start of URL
- **End**: Jump to end of URL
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **Any character**: Insert at cursor position

#### Response Viewer (when focused)

- **↑/↓**: Scroll response one line at a time
- **Page Up/Page Down**: Scroll response 10 lines at a time
- **Home**: Jump to top of response

## Dependencies

- **ratatui** (0.30): Terminal UI framework
- **crossterm** (0.29): Cross-platform terminal manipulation
- **reqwest** (0.12): HTTP client with blocking support
- **anyhow** (1.0): Error handling
- **serde_json** (1.0): JSON parsing and formatting

## Default URL

The application starts with a default URL (`https://pokeapi.co/api/v2/pokemon/snorlax`) to help you test the functionality immediately.

## Response Formatting

- **JSON responses**: Automatically parsed and pretty-printed
- **Status codes**: Displayed at the top of the response
- **Error handling**: Network errors and parsing errors are displayed in the response area

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contributing

Not accepting contributors at this time