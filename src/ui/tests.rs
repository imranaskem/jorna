use super::*;
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_ui_renders_without_panic() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render without error");
}

#[test]
fn test_ui_renders_with_method_selector_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with method selector focused");
}

#[test]
fn test_ui_renders_with_url_input_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with URL input focused");
}

#[test]
fn test_ui_renders_with_response_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::Response;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with response focused");
}

#[test]
fn test_ui_renders_while_loading() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.loading = true;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render while loading");
}

#[test]
fn test_ui_renders_with_different_methods() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    for method in &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"] {
        let mut app = App::new();
        app.http_method = method.to_string();

        terminal
            .draw(|f| ui(f, &app))
            .unwrap_or_else(|_| panic!("UI should render with {} method", method));
    }
}

#[test]
fn test_ui_renders_with_empty_url() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.url_input = "".to_string();
    app.cursor_position = 0;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with empty URL");
}

#[test]
fn test_ui_renders_with_long_url() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.url_input =
        "https://example.com/very/long/path/with/many/segments/to/test/rendering".to_string();
    app.cursor_position = app.url_input.len();

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with long URL");
}

#[test]
fn test_ui_renders_with_long_response() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.response = "Line 1\n".repeat(100);

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with long response");
}

#[test]
fn test_ui_renders_with_scroll_offset() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.response = "Line 1\n".repeat(100);
    app.response_scroll = 50;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with scroll offset");
}

#[test]
fn test_ui_renders_with_cursor_at_start() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "https://example.com".to_string();
    app.cursor_position = 0;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with cursor at start");
}

#[test]
fn test_ui_renders_with_cursor_in_middle() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "https://example.com".to_string();
    app.cursor_position = 10;

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render with cursor in middle");
}

#[test]
fn test_ui_renders_with_small_terminal() {
    let backend = TestBackend::new(40, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render in small terminal");
}

#[test]
fn test_ui_renders_with_large_terminal() {
    let backend = TestBackend::new(200, 60);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    terminal
        .draw(|f| ui(f, &app))
        .expect("UI should render in large terminal");
}

#[test]
fn test_ui_renders_all_http_methods() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    for (index, method) in crate::app::METHODS.iter().enumerate() {
        let mut app = App::new();
        app.method_index = index;
        app.http_method = method.to_string();

        terminal
            .draw(|f| ui(f, &app))
            .unwrap_or_else(|_| panic!("UI should render with method index {}", index));
    }
}
