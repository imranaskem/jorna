use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::{self, stdout};

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppFocus {
    UrlInput,
    Response,
}

struct App {
    url_input: String,
    cursor_position: usize,
    response: String,
    response_scroll: u16,
    loading: bool,
    focus: AppFocus,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let default_url = "https://pokeapi.co/api/v2/pokemon/snorlax".to_string();
        let cursor_pos = default_url.len();

        Self {
            url_input: default_url,
            cursor_position: cursor_pos,
            response: "Response will appear here...".to_string(),
            response_scroll: 0,
            loading: false,
            focus: AppFocus::UrlInput,
            should_quit: false,
        }
    }

    fn send_request(&mut self) {
        if self.url_input.is_empty() {
            self.response = "Error: URL cannot be empty".to_string();
            return;
        }

        let url = self.url_input.clone();
        self.loading = true;
        self.response = "Loading...".to_string();
        self.response_scroll = 0;

        // Simple synchronous request - same as original implementation
        let response_text = match reqwest::blocking::get(&url) {
            Ok(response) => {
                let status = response.status();
                let headers = format!("Status: {}\n\n", status);
                match response.text() {
                    Ok(body) => {
                        // Try to parse and pretty-print JSON
                        let formatted_body = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            serde_json::to_string_pretty(&json).unwrap_or(body)
                        } else {
                            body
                        };
                        format!("{}{}", headers, formatted_body)
                    }
                    Err(e) => format!("Error reading response: {}", e),
                }
            }
            Err(e) => format!("Request failed: {}", e),
        };

        self.response = response_text;
        self.loading = false;
    }

    fn handle_input_char(&mut self, c: char) {
        self.url_input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.url_input.remove(self.cursor_position);
        }
    }

    fn handle_delete(&mut self) {
        if self.cursor_position < self.url_input.len() {
            self.url_input.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.url_input.len() {
            self.cursor_position += 1;
        }
    }

    fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.url_input.len();
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // URL input
            Constraint::Length(1),  // Instructions
            Constraint::Min(10),    // Response area
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("HTTP Client")
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // URL Input with cursor
    let input_text = if app.focus == AppFocus::UrlInput {
        // Show cursor when focused
        let before_cursor = &app.url_input[..app.cursor_position];
        let after_cursor = &app.url_input[app.cursor_position..];

        Line::from(vec![
            Span::raw(before_cursor),
            Span::styled("█", Style::default().fg(Color::Cyan)),
            Span::raw(after_cursor),
        ])
    } else {
        Line::from(app.url_input.as_str())
    };

    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("URL")
                .border_style(if app.focus == AppFocus::UrlInput {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(input_widget, chunks[1]);

    // Instructions
    let instructions = if app.loading {
        "Loading..."
    } else {
        "Enter: Send | Tab: Switch Focus | ↑↓: Scroll Response | Esc: Quit"
    };
    let instructions_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(instructions_widget, chunks[2]);

    // Response
    let response_block = Block::default()
        .borders(Borders::ALL)
        .title("Response")
        .border_style(if app.focus == AppFocus::Response {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        });

    let response_widget = Paragraph::new(app.response.as_str())
        .block(response_block)
        .wrap(Wrap { trim: false })
        .scroll((app.response_scroll, 0))
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(response_widget, chunks[3]);
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    // Global keybindings
    match key.code {
        KeyCode::Esc => {
            app.should_quit = true;
            return;
        }
        KeyCode::Tab => {
            app.focus = match app.focus {
                AppFocus::UrlInput => AppFocus::Response,
                AppFocus::Response => AppFocus::UrlInput,
            };
            return;
        }
        _ => {}
    }

    // Context-specific keybindings
    match app.focus {
        AppFocus::UrlInput => {
            if !app.loading {
                match key.code {
                    KeyCode::Enter => {
                        app.send_request();
                    }
                    KeyCode::Char(c) => {
                        app.handle_input_char(c);
                    }
                    KeyCode::Backspace => {
                        app.handle_backspace();
                    }
                    KeyCode::Delete => {
                        app.handle_delete();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Home => {
                        app.move_cursor_to_start();
                    }
                    KeyCode::End => {
                        app.move_cursor_to_end();
                    }
                    _ => {}
                }
            }
        }
        AppFocus::Response => {
            match key.code {
                KeyCode::Up => {
                    app.response_scroll = app.response_scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    app.response_scroll = app.response_scroll.saturating_add(1);
                }
                KeyCode::PageUp => {
                    app.response_scroll = app.response_scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    app.response_scroll = app.response_scroll.saturating_add(10);
                }
                KeyCode::Home => {
                    app.response_scroll = 0;
                }
                _ => {}
            }
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if app.should_quit {
            break;
        }

        // Poll for events with timeout
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only handle KeyPress events (ignore KeyRelease)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                handle_key_event(&mut app, key);
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Setup panic hook for guaranteed cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Initialize app state
    let app = App::new();

    // Run app with proper error handling
    let result = run_app(&mut terminal, app);

    // Cleanup terminal (even on error)
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    // Return error after cleanup
    result?;

    Ok(())
}
