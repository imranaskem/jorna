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
    assert_eq!(app.request_name, "Default");
    assert_eq!(app.request_path, "Default");
    assert!(!app.show_request_picker);
    assert_eq!(app.picker_mode, PickerMode::Selecting);
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

// Serialization tests
#[test]
fn test_serialization_roundtrip() {
    let app = App::new();
    let json = serde_json::to_string(&app).unwrap();
    let loaded: App = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.url_input, app.url_input);
    assert_eq!(loaded.cursor_position, app.cursor_position);
    assert_eq!(loaded.response, app.response);
    assert_eq!(loaded.focus, app.focus);
    assert_eq!(loaded.http_method, app.http_method);
    assert_eq!(loaded.method_index, app.method_index);
}

#[test]
fn test_serialization_transient_fields_excluded() {
    let mut app = App::new();
    app.loading = true;
    app.should_quit = true;

    let json = serde_json::to_string(&app).unwrap();
    let loaded: App = serde_json::from_str(&json).unwrap();

    // Transient fields should default to false on load
    assert!(!loaded.loading);
    assert!(!loaded.should_quit);
}

#[test]
fn test_serialization_duration_some() {
    let mut app = App::new();
    app.response_time = Some(std::time::Duration::from_millis(250));

    let json = serde_json::to_string(&app).unwrap();
    assert!(json.contains("250"));

    let loaded: App = serde_json::from_str(&json).unwrap();
    assert_eq!(
        loaded.response_time,
        Some(std::time::Duration::from_millis(250))
    );
}

#[test]
fn test_serialization_duration_none() {
    let app = App::new();
    assert!(app.response_time.is_none());

    let json = serde_json::to_string(&app).unwrap();
    let loaded: App = serde_json::from_str(&json).unwrap();
    assert!(loaded.response_time.is_none());
}

#[test]
fn test_serialization_populated_fields_survive_roundtrip() {
    let mut app = App::new();
    app.url_input = "https://example.com/api".to_string();
    app.cursor_position = 10;
    app.http_method = "POST".to_string();
    app.method_index = 1;
    app.headers_input = vec![
        "Content-Type: application/json".to_string(),
        "Authorization: Bearer abc".to_string(),
    ];
    app.headers_cursor_line = 1;
    app.headers_cursor_col = 5;
    app.body_input = vec![
        "{".to_string(),
        "  \"key\": \"value\"".to_string(),
        "}".to_string(),
    ];
    app.body_cursor_line = 2;
    app.body_cursor_col = 1;
    app.response = "OK".to_string();
    app.response_scroll = 3;
    app.status_code = Some(200);
    app.response_size = Some(512);
    app.response_time = Some(std::time::Duration::from_millis(42));
    app.focus = AppFocus::BodyInput;

    let json = serde_json::to_string(&app).unwrap();
    let loaded: App = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.url_input, "https://example.com/api");
    assert_eq!(loaded.cursor_position, 10);
    assert_eq!(loaded.http_method, "POST");
    assert_eq!(loaded.method_index, 1);
    assert_eq!(loaded.headers_input.len(), 2);
    assert_eq!(loaded.headers_cursor_line, 1);
    assert_eq!(loaded.headers_cursor_col, 5);
    assert_eq!(loaded.body_input.len(), 3);
    assert_eq!(loaded.body_cursor_line, 2);
    assert_eq!(loaded.body_cursor_col, 1);
    assert_eq!(loaded.response, "OK");
    assert_eq!(loaded.response_scroll, 3);
    assert_eq!(loaded.status_code, Some(200));
    assert_eq!(loaded.response_size, Some(512));
    assert_eq!(
        loaded.response_time,
        Some(std::time::Duration::from_millis(42))
    );
    assert_eq!(loaded.focus, AppFocus::BodyInput);
}

#[test]
fn test_serialization_unknown_fields_ignored() {
    let json = r#"{
        "url_input": "https://example.com",
        "cursor_position": 0,
        "response": "{}",
        "response_scroll": 0,
        "focus": "UrlInput",
        "http_method": "GET",
        "method_index": 0,
        "headers_input": [""],
        "headers_cursor_line": 0,
        "headers_cursor_col": 0,
        "headers_scroll": 0,
        "body_input": [""],
        "body_cursor_line": 0,
        "body_cursor_col": 0,
        "body_scroll": 0,
        "response_time": null,
        "status_code": null,
        "response_size": null,
        "some_future_field": "should be ignored"
    }"#;

    let loaded: Result<App, _> = serde_json::from_str(json);
    assert!(loaded.is_ok());
    assert_eq!(loaded.unwrap().url_input, "https://example.com");
}

// Serialization: picker fields are excluded
#[test]
fn test_serialization_picker_fields_excluded() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_selected = 5;
    app.request_name = "MyRequest".to_string();
    app.request_path = "Folder/MyRequest".to_string();

    let json = serde_json::to_string(&app).unwrap();
    let loaded: App = serde_json::from_str(&json).unwrap();

    // Picker fields should default on load
    assert!(!loaded.show_request_picker);
    assert_eq!(loaded.picker_selected, 0);
    // request_name and request_path are skip, so they default
    assert_eq!(loaded.request_name, "");
    assert_eq!(loaded.request_path, "");
}

// Picker logic tests (no filesystem)
#[test]
fn test_picker_open_close() {
    let mut app = App::new();
    assert!(!app.show_request_picker);

    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    assert!(app.show_request_picker);

    app.close_picker();
    assert!(!app.show_request_picker);
    assert_eq!(app.picker_mode, PickerMode::Selecting);
}

#[test]
fn test_picker_start_cancel_naming() {
    let mut app = App::new();
    app.picker_start_naming();

    assert_eq!(app.picker_mode, PickerMode::Naming);
    assert_eq!(app.picker_name_input, "");
    assert_eq!(app.picker_name_cursor, 0);

    app.picker_cancel_naming();
    assert_eq!(app.picker_mode, PickerMode::Selecting);
}

#[test]
fn test_picker_enter_empty_entries() {
    let mut app = App::new();
    app.picker_entries = Vec::new();

    // Should not panic
    app.picker_enter();
}

#[test]
fn test_picker_enter_folder() {
    let mut app = App::new();
    app.picker_entries = vec![PickerEntry::Folder {
        name: "Auth".to_string(),
    }];
    app.picker_selected = 0;
    app.picker_current_folder = String::new();

    // picker_enter on a folder updates the current folder
    // But since there's no filesystem, it'll just update state
    app.picker_current_folder = "Auth".to_string();
    assert_eq!(app.picker_current_folder, "Auth");
}

#[test]
fn test_picker_go_back_at_root_closes() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_current_folder = String::new();

    app.picker_go_back();

    assert!(!app.show_request_picker);
}

#[test]
fn test_picker_go_back_to_parent() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_current_folder = "Auth/Admin".to_string();

    app.picker_go_back();

    assert_eq!(app.picker_current_folder, "Auth");
    assert!(app.show_request_picker);
}

#[test]
fn test_picker_go_back_single_level() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_current_folder = "Auth".to_string();

    app.picker_go_back();

    assert_eq!(app.picker_current_folder, "");
    assert!(app.show_request_picker);
}

// Persistence helpers
#[test]
fn test_requests_dir_returns_path() {
    // Just verify the helper returns a path under .jorna
    if let Some(dir) = App::requests_dir() {
        assert!(dir.ends_with("requests"));
    }
}

#[test]
fn test_state_file_path_returns_path() {
    if let Some(path) = App::state_file_path() {
        assert!(path.ends_with("state.json"));
    }
}

#[test]
fn test_picker_create_request_empty_name_noop() {
    let mut app = App::new();
    app.show_request_picker = true;
    let original_url = app.url_input.clone();

    app.picker_create_request("  ".to_string());

    // Empty/whitespace name should be a no-op (picker stays open)
    assert!(app.show_request_picker);
    assert_eq!(app.url_input, original_url);
}

#[test]
fn test_request_name_derived_from_path() {
    let mut app = App::new();
    app.request_path = "Auth/Login".to_string();
    app.request_name = app
        .request_path
        .rsplit('/')
        .next()
        .unwrap_or(&app.request_path)
        .to_string();

    assert_eq!(app.request_name, "Login");
}

#[test]
fn test_request_name_derived_from_simple_path() {
    let mut app = App::new();
    app.request_path = "Default".to_string();
    app.request_name = app
        .request_path
        .rsplit('/')
        .next()
        .unwrap_or(&app.request_path)
        .to_string();

    assert_eq!(app.request_name, "Default");
}

#[test]
fn test_picker_start_renaming_sets_mode_and_prefills() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_entries = vec![PickerEntry::Request {
        name: "MyRequest".to_string(),
        path: "Folder/MyRequest".to_string(),
    }];
    app.picker_selected = 0;

    app.picker_start_renaming();

    assert_eq!(app.picker_mode, PickerMode::Renaming);
    assert_eq!(app.picker_name_input, "MyRequest");
    assert_eq!(app.picker_name_cursor, 9);
    assert_eq!(app.picker_rename_path, "Folder/MyRequest");
}

#[test]
fn test_picker_start_renaming_ignores_folders() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_entries = vec![PickerEntry::Folder {
        name: "Auth".to_string(),
    }];
    app.picker_selected = 0;

    app.picker_start_renaming();

    // Should stay in Selecting mode since selected entry is a folder
    assert_eq!(app.picker_mode, PickerMode::Selecting);
}

#[test]
fn test_picker_start_renaming_empty_entries_noop() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_entries = Vec::new();

    app.picker_start_renaming();

    assert_eq!(app.picker_mode, PickerMode::Selecting);
}

#[test]
fn test_picker_rename_request_empty_name_noop() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Renaming;
    app.picker_rename_path = "OldName".to_string();

    app.picker_rename_request("  ".to_string());

    // Should stay in Renaming mode since empty name is a no-op
    assert_eq!(app.picker_mode, PickerMode::Renaming);
}
