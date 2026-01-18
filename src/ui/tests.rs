use super::*;
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_ui_renders_without_panic() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render without error");
}

#[test]
fn test_ui_renders_with_method_selector_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with method selector focused");
}

#[test]
fn test_ui_renders_with_url_input_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with URL input focused");
}

#[test]
fn test_ui_renders_with_response_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::Response;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with response focused");
}

#[test]
fn test_ui_renders_while_loading() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.loading = true;

    terminal
        .draw(|f| ui(f, &mut app))
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
            .draw(|f| ui(f, &mut app))
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
        .draw(|f| ui(f, &mut app))
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
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with long URL");
}

#[test]
fn test_ui_renders_with_long_response() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.response = "Line 1\n".repeat(100);

    terminal
        .draw(|f| ui(f, &mut app))
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
        .draw(|f| ui(f, &mut app))
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
        .draw(|f| ui(f, &mut app))
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
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with cursor in middle");
}

#[test]
fn test_ui_renders_with_small_terminal() {
    let backend = TestBackend::new(40, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render in small terminal");
}

#[test]
fn test_ui_renders_with_large_terminal() {
    let backend = TestBackend::new(200, 60);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    terminal
        .draw(|f| ui(f, &mut app))
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
            .draw(|f| ui(f, &mut app))
            .unwrap_or_else(|_| panic!("UI should render with method index {}", index));
    }
}

// Headers and Body widget rendering tests
#[test]
fn test_ui_renders_with_headers_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with headers input focused");
}

#[test]
fn test_ui_renders_with_body_focused() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with body input focused");
}

#[test]
fn test_ui_renders_with_headers_content() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.headers_input = vec![
        "Content-Type: application/json".to_string(),
        "Authorization: Bearer token123".to_string(),
    ];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with headers content");
}

#[test]
fn test_ui_renders_with_body_content() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.body_input = vec![
        "{".to_string(),
        "  \"name\": \"test\",".to_string(),
        "  \"value\": 123".to_string(),
        "}".to_string(),
    ];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with body content");
}

#[test]
fn test_ui_renders_headers_with_cursor() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 3;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render headers with cursor");
}

#[test]
fn test_ui_renders_body_with_cursor() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{}".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 1;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render body with cursor");
}

#[test]
fn test_ui_renders_with_multiline_headers() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.headers_input = vec![
        "Header1: value1".to_string(),
        "Header2: value2".to_string(),
        "Header3: value3".to_string(),
        "Header4: value4".to_string(),
    ];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with multiple header lines");
}

#[test]
fn test_ui_renders_with_multiline_body() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.body_input = vec![
        "{".to_string(),
        "  \"field1\": \"value1\",".to_string(),
        "  \"field2\": \"value2\",".to_string(),
        "  \"nested\": {".to_string(),
        "    \"key\": \"value\"".to_string(),
        "  }".to_string(),
        "}".to_string(),
    ];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with multiple body lines");
}

#[test]
fn test_ui_renders_with_empty_headers() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.headers_input = vec!["".to_string()];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with empty headers");
}

#[test]
fn test_ui_renders_with_empty_body() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.body_input = vec!["".to_string()];

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with empty body");
}

#[test]
fn test_ui_renders_all_focus_states() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let focus_states = vec![
        AppFocus::MethodSelector,
        AppFocus::UrlInput,
        AppFocus::HeadersInput,
        AppFocus::BodyInput,
        AppFocus::Response,
    ];

    for focus in focus_states {
        let mut app = App::new();
        app.focus = focus;

        terminal
            .draw(|f| ui(f, &mut app))
            .unwrap_or_else(|_| panic!("UI should render with focus state {:?}", focus));
    }
}

// Status line tests
#[test]
fn test_ui_renders_status_line_empty_initially() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    // No response metadata set
    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with empty status line");
}

#[test]
fn test_ui_renders_status_line_with_metadata() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.status_code = Some(200);
    app.response_time = Some(std::time::Duration::from_millis(150));
    app.response_size = Some(1024);

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with status line metadata");
}

#[test]
fn test_ui_renders_status_line_loading() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.loading = true;

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with loading status line");
}

#[test]
fn test_ui_renders_status_line_with_large_response_time() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.status_code = Some(200);
    app.response_time = Some(std::time::Duration::from_millis(2500));
    app.response_size = Some(500);

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with large response time (seconds format)");
}

#[test]
fn test_ui_renders_status_line_with_large_response_size() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.status_code = Some(200);
    app.response_time = Some(std::time::Duration::from_millis(100));
    app.response_size = Some(2 * 1024 * 1024); // 2MB

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with large response size (MB format)");
}

#[test]
fn test_ui_renders_status_line_with_error_status() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.status_code = Some(404);
    app.response_time = Some(std::time::Duration::from_millis(50));
    app.response_size = Some(100);

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with error status code");
}

#[test]
fn test_ui_renders_status_line_with_only_status_code() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.status_code = Some(200);

    terminal
        .draw(|f| ui(f, &mut app))
        .expect("UI should render with only status code");
}
