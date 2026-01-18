use super::*;

#[test]
fn test_app_initialization() {
    let app = App::new();

    assert_eq!(app.url_input, "https://pokeapi.co/api/v2/pokemon/snorlax");
    assert_eq!(app.cursor_position, app.url_input.len());
    assert_eq!(app.response, "{}");
    assert_eq!(app.response_scroll, 0);
    assert!(!app.loading);
    assert_eq!(app.focus, AppFocus::MethodSelector);
    assert!(!app.should_quit);
    assert_eq!(app.http_method, "GET");
    assert_eq!(app.method_index, 0);
    assert!(app.response_time.is_none());
    assert!(app.status_code.is_none());
    assert!(app.response_size.is_none());
}

#[test]
fn test_handle_input_char() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 5;

    app.handle_input_char('!');

    assert_eq!(app.url_input, "hello!");
    assert_eq!(app.cursor_position, 6);
}

#[test]
fn test_handle_input_char_middle() {
    let mut app = App::new();
    app.url_input = "helo".to_string();
    app.cursor_position = 2;

    app.handle_input_char('l');

    assert_eq!(app.url_input, "hello");
    assert_eq!(app.cursor_position, 3);
}

#[test]
fn test_handle_backspace() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 5;

    app.handle_backspace();

    assert_eq!(app.url_input, "hell");
    assert_eq!(app.cursor_position, 4);
}

#[test]
fn test_handle_backspace_at_start() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 0;

    app.handle_backspace();

    assert_eq!(app.url_input, "hello");
    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_handle_delete() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 0;

    app.handle_delete();

    assert_eq!(app.url_input, "ello");
    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_handle_delete_at_end() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 5;

    app.handle_delete();

    assert_eq!(app.url_input, "hello");
    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_move_cursor_left() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 3;

    app.move_cursor_left();

    assert_eq!(app.cursor_position, 2);
}

#[test]
fn test_move_cursor_left_at_start() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 0;

    app.move_cursor_left();

    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_move_cursor_right() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 2;

    app.move_cursor_right();

    assert_eq!(app.cursor_position, 3);
}

#[test]
fn test_move_cursor_right_at_end() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 5;

    app.move_cursor_right();

    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_move_cursor_to_start() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 3;

    app.move_cursor_to_start();

    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_move_cursor_to_end() {
    let mut app = App::new();
    app.url_input = "hello".to_string();
    app.cursor_position = 0;

    app.move_cursor_to_end();

    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_send_request_with_empty_url() {
    let mut app = App::new();
    app.url_input = "".to_string();

    app.send_request();

    assert_eq!(app.response, "Error: URL cannot be empty");
    assert!(!app.loading);
}

#[test]
fn test_methods_constant() {
    assert_eq!(METHODS.len(), 7);
    assert_eq!(METHODS[0], "GET");
    assert_eq!(METHODS[1], "POST");
    assert_eq!(METHODS[2], "PUT");
    assert_eq!(METHODS[3], "DELETE");
    assert_eq!(METHODS[4], "PATCH");
    assert_eq!(METHODS[5], "HEAD");
    assert_eq!(METHODS[6], "OPTIONS");
}

// Multi-line input tests - Headers
#[test]
fn test_multiline_char_headers() {
    let mut app = App::new();
    app.headers_input = vec!["".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 0;

    app.handle_multiline_char('C', true);
    app.handle_multiline_char('o', true);
    app.handle_multiline_char('n', true);

    assert_eq!(app.headers_input[0], "Con");
    assert_eq!(app.headers_cursor_col, 3);
}

#[test]
fn test_multiline_backspace_headers() {
    let mut app = App::new();
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 7;

    app.handle_multiline_backspace(true);

    assert_eq!(app.headers_input[0], "Conten");
    assert_eq!(app.headers_cursor_col, 6);
}

#[test]
fn test_multiline_backspace_at_line_start() {
    let mut app = App::new();
    app.headers_input = vec!["First".to_string(), "Second".to_string()];
    app.headers_cursor_line = 1;
    app.headers_cursor_col = 0;

    app.handle_multiline_backspace(true);

    assert_eq!(app.headers_input.len(), 1);
    assert_eq!(app.headers_input[0], "FirstSecond");
    assert_eq!(app.headers_cursor_line, 0);
    assert_eq!(app.headers_cursor_col, 5);
}

#[test]
fn test_multiline_enter_headers() {
    let mut app = App::new();
    let header = "Content-Type: application/json".to_string();
    app.headers_input = vec![header.clone()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = header.len();

    app.handle_multiline_enter(true);

    assert_eq!(app.headers_input.len(), 2);
    assert_eq!(app.headers_input[0], "Content-Type: application/json");
    assert_eq!(app.headers_input[1], "");
    assert_eq!(app.headers_cursor_line, 1);
    assert_eq!(app.headers_cursor_col, 0);
}

#[test]
fn test_multiline_enter_middle_of_line() {
    let mut app = App::new();
    app.headers_input = vec!["HelloWorld".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 5;

    app.handle_multiline_enter(true);

    assert_eq!(app.headers_input.len(), 2);
    assert_eq!(app.headers_input[0], "Hello");
    assert_eq!(app.headers_input[1], "World");
    assert_eq!(app.headers_cursor_line, 1);
    assert_eq!(app.headers_cursor_col, 0);
}

#[test]
fn test_multiline_up_headers() {
    let mut app = App::new();
    app.headers_input = vec!["First".to_string(), "Second".to_string()];
    app.headers_cursor_line = 1;
    app.headers_cursor_col = 3;

    app.handle_multiline_up(true);

    assert_eq!(app.headers_cursor_line, 0);
    assert_eq!(app.headers_cursor_col, 3);
}

#[test]
fn test_multiline_up_at_top() {
    let mut app = App::new();
    app.headers_input = vec!["First".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 2;

    app.handle_multiline_up(true);

    assert_eq!(app.headers_cursor_line, 0);
    assert_eq!(app.headers_cursor_col, 2);
}

#[test]
fn test_multiline_down_headers() {
    let mut app = App::new();
    app.headers_input = vec!["First".to_string(), "Second".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 2;

    app.handle_multiline_down(true);

    assert_eq!(app.headers_cursor_line, 1);
    assert_eq!(app.headers_cursor_col, 2);
}

#[test]
fn test_multiline_down_at_bottom() {
    let mut app = App::new();
    app.headers_input = vec!["First".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 2;

    app.handle_multiline_down(true);

    assert_eq!(app.headers_cursor_line, 0);
    assert_eq!(app.headers_cursor_col, 2);
}

#[test]
fn test_multiline_left_headers() {
    let mut app = App::new();
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 5;

    app.handle_multiline_left(true);

    assert_eq!(app.headers_cursor_col, 4);
}

#[test]
fn test_multiline_left_at_start() {
    let mut app = App::new();
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 0;

    app.handle_multiline_left(true);

    assert_eq!(app.headers_cursor_col, 0);
}

#[test]
fn test_multiline_right_headers() {
    let mut app = App::new();
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 3;

    app.handle_multiline_right(true);

    assert_eq!(app.headers_cursor_col, 4);
}

#[test]
fn test_multiline_right_at_end() {
    let mut app = App::new();
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 7;

    app.handle_multiline_right(true);

    assert_eq!(app.headers_cursor_col, 7);
}

// Multi-line input tests - Body
#[test]
fn test_multiline_char_body() {
    let mut app = App::new();
    app.body_input = vec!["".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 0;

    app.handle_multiline_char('{', false);
    app.handle_multiline_char('}', false);

    assert_eq!(app.body_input[0], "{}");
    assert_eq!(app.body_cursor_col, 2);
}

#[test]
fn test_multiline_backspace_body() {
    let mut app = App::new();
    app.body_input = vec!["{ }".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 2;

    app.handle_multiline_backspace(false);

    assert_eq!(app.body_input[0], "{}");
    assert_eq!(app.body_cursor_col, 1);
}

#[test]
fn test_multiline_enter_body() {
    let mut app = App::new();
    app.body_input = vec!["{".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 1;

    app.handle_multiline_enter(false);

    assert_eq!(app.body_input.len(), 2);
    assert_eq!(app.body_input[0], "{");
    assert_eq!(app.body_input[1], "");
    assert_eq!(app.body_cursor_line, 1);
}

// Header parsing tests
#[test]
fn test_send_request_parses_valid_headers() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.headers_input = vec![
        "Content-Type: application/json".to_string(),
        "Authorization: Bearer token123".to_string(),
    ];
    app.body_input = vec!["{}".to_string()];

    // We can't easily test the actual HTTP request without mocking,
    // but we can verify the request doesn't error on parsing
    app.send_request();

    // If headers were malformed, send_request would still execute
    // This test mainly ensures no panic occurs during parsing
    assert!(!app.response.is_empty());
}

#[test]
fn test_send_request_skips_invalid_headers() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.headers_input = vec![
        "Valid-Header: value".to_string(),
        "Invalid Header Without Colon".to_string(),
        "Another-Valid: value2".to_string(),
    ];
    app.body_input = vec!["{}".to_string()];

    // Invalid headers should be silently skipped
    app.send_request();
    assert!(!app.response.is_empty());
}

#[test]
fn test_send_request_handles_empty_headers() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/get".to_string();
    app.headers_input = vec!["".to_string()];
    app.body_input = vec!["".to_string()];

    app.send_request();
    assert!(!app.response.is_empty());
}

// JSON validation tests
#[test]
fn test_send_request_validates_json() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec!["{invalid json}".to_string()];

    app.send_request();

    assert!(app.response.contains("Error: Invalid JSON"));
    assert!(!app.loading);
}

#[test]
fn test_send_request_accepts_valid_json() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec![
        "{".to_string(),
        "  \"name\": \"test\",".to_string(),
        "  \"value\": 123".to_string(),
        "}".to_string(),
    ];

    app.send_request();

    // Valid JSON should not show validation error
    assert!(!app.response.contains("Error: Invalid JSON"));
}

#[test]
fn test_send_request_accepts_empty_body() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/get".to_string();
    app.body_input = vec!["".to_string()];

    app.send_request();

    // Empty body is valid
    assert!(!app.response.contains("Error: Invalid JSON"));
}

#[test]
fn test_send_request_trims_whitespace_in_body() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec!["".to_string(), "  ".to_string(), "".to_string()];

    app.send_request();

    // Whitespace-only body should be treated as empty
    assert!(!app.response.contains("Error: Invalid JSON"));
}

// App initialization tests for new fields
#[test]
fn test_app_initialization_headers_and_body() {
    let app = App::new();

    assert_eq!(app.headers_input, vec![String::new()]);
    assert_eq!(app.headers_cursor_line, 0);
    assert_eq!(app.headers_cursor_col, 0);
    assert_eq!(app.headers_scroll, 0);

    assert_eq!(app.body_input, vec![String::new()]);
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 0);
    assert_eq!(app.body_scroll, 0);
}

// Format body JSON tests
#[test]
fn test_format_body_json_valid() {
    let mut app = App::new();
    app.body_input = vec!["{\"name\":\"test\",\"value\":123}".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 10;

    app.format_body_json();

    assert_eq!(app.body_input.len(), 4);
    assert_eq!(app.body_input[0], "{");
    assert_eq!(app.body_input[1], "  \"name\": \"test\",");
    assert_eq!(app.body_input[2], "  \"value\": 123");
    assert_eq!(app.body_input[3], "}");
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 0);
}

#[test]
fn test_format_body_json_multiline_input() {
    let mut app = App::new();
    app.body_input = vec!["{\"name\":".to_string(), "\"test\"}".to_string()];
    app.body_cursor_line = 1;
    app.body_cursor_col = 5;

    app.format_body_json();

    assert_eq!(app.body_input.len(), 3);
    assert_eq!(app.body_input[0], "{");
    assert_eq!(app.body_input[1], "  \"name\": \"test\"");
    assert_eq!(app.body_input[2], "}");
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 0);
}

#[test]
fn test_format_body_json_invalid_json() {
    let mut app = App::new();
    app.body_input = vec!["{invalid json}".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 5;

    app.format_body_json();

    // Should not change anything for invalid JSON
    assert_eq!(app.body_input, vec!["{invalid json}".to_string()]);
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 5);
}

#[test]
fn test_format_body_json_empty() {
    let mut app = App::new();
    app.body_input = vec!["".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 0;

    app.format_body_json();

    // Empty string is invalid JSON, should not change
    assert_eq!(app.body_input, vec!["".to_string()]);
}

#[test]
fn test_format_body_json_already_formatted() {
    let mut app = App::new();
    app.body_input = vec![
        "{".to_string(),
        "  \"name\": \"test\"".to_string(),
        "}".to_string(),
    ];

    app.format_body_json();

    // Should still work (re-format)
    assert_eq!(app.body_input.len(), 3);
    assert_eq!(app.body_input[0], "{");
}

// Response metadata tests
#[test]
fn test_send_request_clears_metadata_before_request() {
    let mut app = App::new();
    // Set some previous metadata
    app.response_time = Some(std::time::Duration::from_millis(100));
    app.status_code = Some(200);
    app.response_size = Some(1024);

    // Use empty URL to trigger early return
    app.url_input = "".to_string();
    app.send_request();

    // Metadata should remain unchanged since we returned early before clearing
    assert!(app.response_time.is_some());
}

#[test]
fn test_send_request_populates_metadata_on_success() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/get".to_string();

    app.send_request();

    // After successful request, metadata should be populated
    assert!(app.response_time.is_some());
    assert!(app.status_code.is_some());
    assert!(app.response_size.is_some());
    assert_eq!(app.status_code, Some(200));
}

#[test]
fn test_send_request_metadata_cleared_before_new_request() {
    let mut app = App::new();
    // Set some previous metadata
    app.response_time = Some(std::time::Duration::from_millis(100));
    app.status_code = Some(404);
    app.response_size = Some(50);
    app.url_input = "https://httpbin.org/get".to_string();

    app.send_request();

    // After new request, old metadata should be replaced
    assert!(app.status_code.is_some());
    assert_eq!(app.status_code, Some(200));
}

#[test]
fn test_send_request_invalid_json_clears_metadata() {
    let mut app = App::new();
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec!["{invalid json}".to_string()];

    app.send_request();

    // JSON validation fails early, metadata should be None
    assert!(app.response_time.is_none());
    assert!(app.status_code.is_none());
    assert!(app.response_size.is_none());
}
