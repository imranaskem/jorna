use super::*;
use crate::app::PickerEntry;
use crossterm::event::KeyModifiers;

fn create_key_event(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn test_esc_quits_app() {
    let mut app = App::new();
    assert!(!app.should_quit);

    handle_key_event(&mut app, create_key_event(KeyCode::Esc));

    assert!(app.should_quit);
}

#[test]
fn test_tab_cycles_focus() {
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));
    assert_eq!(app.focus, AppFocus::UrlInput);

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));
    assert_eq!(app.focus, AppFocus::HeadersInput);

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));
    assert_eq!(app.focus, AppFocus::BodyInput);

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));
    assert_eq!(app.focus, AppFocus::Response);

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));
    assert_eq!(app.focus, AppFocus::MethodSelector);
}

#[test]
fn test_method_selector_up_cycles_methods() {
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;
    app.method_index = 0;
    app.http_method = "GET".to_string();

    handle_key_event(&mut app, create_key_event(KeyCode::Up));

    assert_eq!(app.method_index, 6);
    assert_eq!(app.http_method, "OPTIONS");
}

#[test]
fn test_method_selector_down_cycles_methods() {
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;
    app.method_index = 6;
    app.http_method = "OPTIONS".to_string();

    handle_key_event(&mut app, create_key_event(KeyCode::Down));

    assert_eq!(app.method_index, 0);
    assert_eq!(app.http_method, "GET");
}

#[test]
fn test_method_selector_down_increments() {
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;
    app.method_index = 0;
    app.http_method = "GET".to_string();

    handle_key_event(&mut app, create_key_event(KeyCode::Down));

    assert_eq!(app.method_index, 1);
    assert_eq!(app.http_method, "POST");
}

#[test]
fn test_url_input_char() {
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "".to_string();
    app.cursor_position = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('h')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('i')));

    assert_eq!(app.url_input, "hi");
    assert_eq!(app.cursor_position, 2);
}

#[test]
fn test_url_input_backspace() {
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "hello".to_string();
    app.cursor_position = 5;

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert_eq!(app.url_input, "hell");
    assert_eq!(app.cursor_position, 4);
}

#[test]
fn test_url_input_delete() {
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "hello".to_string();
    app.cursor_position = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Delete));

    assert_eq!(app.url_input, "ello");
    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_url_input_cursor_movement() {
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "hello".to_string();
    app.cursor_position = 2;

    handle_key_event(&mut app, create_key_event(KeyCode::Left));
    assert_eq!(app.cursor_position, 1);

    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.cursor_position, 2);

    handle_key_event(&mut app, create_key_event(KeyCode::Home));
    assert_eq!(app.cursor_position, 0);

    handle_key_event(&mut app, create_key_event(KeyCode::End));
    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_response_scroll() {
    let mut app = App::new();
    app.focus = AppFocus::Response;
    app.response_scroll = 5;

    handle_key_event(&mut app, create_key_event(KeyCode::Up));
    assert_eq!(app.response_scroll, 4);

    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.response_scroll, 5);

    handle_key_event(&mut app, create_key_event(KeyCode::PageUp));
    assert_eq!(app.response_scroll, 0);

    app.response_scroll = 5;
    handle_key_event(&mut app, create_key_event(KeyCode::PageDown));
    assert_eq!(app.response_scroll, 15);

    handle_key_event(&mut app, create_key_event(KeyCode::Home));
    assert_eq!(app.response_scroll, 0);
}

#[test]
fn test_loading_blocks_input() {
    let mut app = App::new();
    app.focus = AppFocus::UrlInput;
    app.url_input = "test".to_string();
    app.cursor_position = 4;
    app.loading = true;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('!')));

    // Should not change because loading is true
    assert_eq!(app.url_input, "test");
    assert_eq!(app.cursor_position, 4);
}

#[test]
fn test_loading_blocks_method_change() {
    let mut app = App::new();
    app.focus = AppFocus::MethodSelector;
    app.method_index = 0;
    app.http_method = "GET".to_string();
    app.loading = true;

    handle_key_event(&mut app, create_key_event(KeyCode::Down));

    // Should not change because loading is true
    assert_eq!(app.method_index, 0);
    assert_eq!(app.http_method, "GET");
}

// Headers input tests
#[test]
fn test_headers_input_char() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('C')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('o')));

    assert_eq!(app.headers_input[0], "Co");
    assert_eq!(app.headers_cursor_col, 2);
}

#[test]
fn test_headers_input_backspace() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 7;

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert_eq!(app.headers_input[0], "Conten");
    assert_eq!(app.headers_cursor_col, 6);
}

#[test]
fn test_headers_input_enter_new_line() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["First".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 5;

    handle_key_event(&mut app, create_key_event(KeyCode::Enter));

    // Regular Enter should create new line in headers
    assert_eq!(app.headers_input.len(), 2);
    assert_eq!(app.headers_input[0], "First");
    assert_eq!(app.headers_input[1], "");
    assert_eq!(app.headers_cursor_line, 1);
}

#[test]
fn test_headers_input_ctrl_s_sends_request() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.url_input = "https://httpbin.org/get".to_string();
    app.headers_input = vec!["Content-Type: application/json".to_string()];

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
    );

    // Ctrl+S should send request
    assert!(!app.response.is_empty());
}

#[test]
fn test_headers_input_ctrl_enter_sends_request() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.url_input = "https://httpbin.org/get".to_string();
    app.headers_input = vec!["Content-Type: application/json".to_string()];

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL),
    );

    // Ctrl+Enter should send request
    assert!(!app.response.is_empty());
}

#[test]
fn test_headers_input_ctrl_t_inserts_two_spaces() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["test".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 0;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL),
    );

    assert_eq!(app.headers_input[0], "  test");
    assert_eq!(app.headers_cursor_col, 2);
}

#[test]
fn test_headers_input_arrow_keys() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["First".to_string(), "Second".to_string()];
    app.headers_cursor_line = 0;
    app.headers_cursor_col = 5;

    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.headers_cursor_line, 1);

    handle_key_event(&mut app, create_key_event(KeyCode::Up));
    assert_eq!(app.headers_cursor_line, 0);

    handle_key_event(&mut app, create_key_event(KeyCode::Left));
    assert_eq!(app.headers_cursor_col, 4);

    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.headers_cursor_col, 5);
}

#[test]
fn test_loading_blocks_headers_input() {
    let mut app = App::new();
    app.focus = AppFocus::HeadersInput;
    app.headers_input = vec!["Content".to_string()];
    app.headers_cursor_col = 7;
    app.loading = true;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('X')));

    // Should not change because loading is true
    assert_eq!(app.headers_input[0], "Content");
}

// Body input tests
#[test]
fn test_body_input_char() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('{')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('}')));

    assert_eq!(app.body_input[0], "{}");
    assert_eq!(app.body_cursor_col, 2);
}

#[test]
fn test_body_input_backspace() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{ }".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 2;

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert_eq!(app.body_input[0], "{}");
    assert_eq!(app.body_cursor_col, 1);
}

#[test]
fn test_body_input_enter_new_line() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 1;

    handle_key_event(&mut app, create_key_event(KeyCode::Enter));

    // Regular Enter should create new line in body
    assert_eq!(app.body_input.len(), 2);
    assert_eq!(app.body_input[0], "{");
    assert_eq!(app.body_input[1], "");
}

#[test]
fn test_body_input_ctrl_enter_sends_request() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec!["{}".to_string()];

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL),
    );

    // Ctrl+Enter should send request
    assert!(!app.response.is_empty());
}

#[test]
fn test_body_input_ctrl_s_sends_request() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.url_input = "https://httpbin.org/post".to_string();
    app.http_method = "POST".to_string();
    app.body_input = vec!["{}".to_string()];

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
    );

    // Ctrl+S should send request
    assert!(!app.response.is_empty());
}

#[test]
fn test_body_input_shift_enter_new_line() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["test".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 4;

    handle_key_event(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT));

    assert_eq!(app.body_input.len(), 2);
    assert_eq!(app.body_input[0], "test");
    assert_eq!(app.body_input[1], "");
}

#[test]
fn test_body_input_arrow_keys() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{".to_string(), "}".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 1;

    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.body_cursor_line, 1);

    handle_key_event(&mut app, create_key_event(KeyCode::Up));
    assert_eq!(app.body_cursor_line, 0);

    handle_key_event(&mut app, create_key_event(KeyCode::Left));
    assert_eq!(app.body_cursor_col, 0);

    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.body_cursor_col, 1);
}

#[test]
fn test_loading_blocks_body_input() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{}".to_string()];
    app.body_cursor_col = 2;
    app.loading = true;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('x')));

    // Should not change because loading is true
    assert_eq!(app.body_input[0], "{}");
}

// Ctrl+T indent tests
#[test]
fn test_body_input_ctrl_t_inserts_two_spaces() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["test".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 0;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL),
    );

    assert_eq!(app.body_input[0], "  test");
    assert_eq!(app.body_cursor_col, 2);
}

#[test]
fn test_body_input_ctrl_t_inserts_at_cursor() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["hello".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 2;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL),
    );

    assert_eq!(app.body_input[0], "he  llo");
    assert_eq!(app.body_cursor_col, 4);
}

// Ctrl+F format JSON tests
#[test]
fn test_body_input_ctrl_f_formats_json() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{\"key\":\"value\"}".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 5;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
    );

    assert_eq!(app.body_input.len(), 3);
    assert_eq!(app.body_input[0], "{");
    assert_eq!(app.body_input[1], "  \"key\": \"value\"");
    assert_eq!(app.body_input[2], "}");
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 0);
}

#[test]
fn test_body_input_ctrl_f_invalid_json_no_change() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["not valid json".to_string()];
    app.body_cursor_line = 0;
    app.body_cursor_col = 5;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
    );

    // Should not change anything
    assert_eq!(app.body_input, vec!["not valid json".to_string()]);
    assert_eq!(app.body_cursor_line, 0);
    assert_eq!(app.body_cursor_col, 5);
}

#[test]
fn test_body_input_ctrl_i_blocked_when_loading() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["test".to_string()];
    app.body_cursor_col = 0;
    app.loading = true;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::CONTROL),
    );

    // Should not change because loading is true
    assert_eq!(app.body_input[0], "test");
    assert_eq!(app.body_cursor_col, 0);
}

#[test]
fn test_body_input_ctrl_f_blocked_when_loading() {
    let mut app = App::new();
    app.focus = AppFocus::BodyInput;
    app.body_input = vec!["{\"key\":\"value\"}".to_string()];
    app.loading = true;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
    );

    // Should not format because loading is true
    assert_eq!(app.body_input, vec!["{\"key\":\"value\"}".to_string()]);
}

// --- Picker event tests ---

#[test]
fn test_ctrl_p_opens_picker() {
    let mut app = App::new();
    assert!(!app.show_request_picker);

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL),
    );

    assert!(app.show_request_picker);
}

#[test]
fn test_ctrl_p_blocked_when_loading() {
    let mut app = App::new();
    app.loading = true;

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL),
    );

    assert!(!app.show_request_picker);
}

#[test]
fn test_picker_esc_closes() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;

    handle_key_event(&mut app, create_key_event(KeyCode::Esc));

    assert!(!app.show_request_picker);
    // Esc should NOT quit when picker is open
    assert!(!app.should_quit);
}

#[test]
fn test_picker_navigation_up_down() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    app.picker_entries = vec![
        PickerEntry::Folder {
            name: "Auth".to_string(),
        },
        PickerEntry::Request {
            name: "Default".to_string(),
            path: "Default".to_string(),
        },
        PickerEntry::Request {
            name: "Test".to_string(),
            path: "Test".to_string(),
        },
    ];
    app.picker_selected = 0;

    // Down
    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.picker_selected, 1);

    // Down again
    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.picker_selected, 2);

    // Down at bottom - stays
    handle_key_event(&mut app, create_key_event(KeyCode::Down));
    assert_eq!(app.picker_selected, 2);

    // Up
    handle_key_event(&mut app, create_key_event(KeyCode::Up));
    assert_eq!(app.picker_selected, 1);

    // Up at top - stays
    app.picker_selected = 0;
    handle_key_event(&mut app, create_key_event(KeyCode::Up));
    assert_eq!(app.picker_selected, 0);
}

#[test]
fn test_picker_j_k_navigation() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    app.picker_entries = vec![
        PickerEntry::Request {
            name: "A".to_string(),
            path: "A".to_string(),
        },
        PickerEntry::Request {
            name: "B".to_string(),
            path: "B".to_string(),
        },
    ];
    app.picker_selected = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('j')));
    assert_eq!(app.picker_selected, 1);

    handle_key_event(&mut app, create_key_event(KeyCode::Char('k')));
    assert_eq!(app.picker_selected, 0);
}

#[test]
fn test_picker_n_starts_naming() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('n')));

    assert_eq!(app.picker_mode, PickerMode::Naming);
}

#[test]
fn test_picker_naming_input() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Naming;
    app.picker_name_input = String::new();
    app.picker_name_cursor = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('T')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('e')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('s')));
    handle_key_event(&mut app, create_key_event(KeyCode::Char('t')));

    assert_eq!(app.picker_name_input, "Test");
    assert_eq!(app.picker_name_cursor, 4);
}

#[test]
fn test_picker_naming_backspace() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Naming;
    app.picker_name_input = "Test".to_string();
    app.picker_name_cursor = 4;

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert_eq!(app.picker_name_input, "Tes");
    assert_eq!(app.picker_name_cursor, 3);
}

#[test]
fn test_picker_naming_esc_cancels() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Naming;
    app.picker_name_input = "Test".to_string();

    handle_key_event(&mut app, create_key_event(KeyCode::Esc));

    assert_eq!(app.picker_mode, PickerMode::Selecting);
    // Picker should stay open
    assert!(app.show_request_picker);
    // App should NOT quit
    assert!(!app.should_quit);
}

#[test]
fn test_picker_naming_cursor_movement() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Naming;
    app.picker_name_input = "Test".to_string();
    app.picker_name_cursor = 4;

    handle_key_event(&mut app, create_key_event(KeyCode::Left));
    assert_eq!(app.picker_name_cursor, 3);

    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.picker_name_cursor, 4);

    // Right at end stays
    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.picker_name_cursor, 4);
}

#[test]
fn test_picker_tab_does_not_cycle_focus() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    app.focus = AppFocus::UrlInput;

    handle_key_event(&mut app, create_key_event(KeyCode::Tab));

    // Focus should NOT change when picker is open
    assert_eq!(app.focus, AppFocus::UrlInput);
}

#[test]
fn test_picker_backspace_at_root_closes() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    app.picker_current_folder = String::new();

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert!(!app.show_request_picker);
}

#[test]
fn test_picker_r_starts_renaming() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Selecting;
    app.picker_entries = vec![PickerEntry::Request {
        name: "Test".to_string(),
        path: "Test".to_string(),
    }];
    app.picker_selected = 0;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('r')));

    assert_eq!(app.picker_mode, PickerMode::Renaming);
    assert_eq!(app.picker_name_input, "Test");
    assert_eq!(app.picker_name_cursor, 4);
}

#[test]
fn test_picker_renaming_esc_cancels() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Renaming;
    app.picker_name_input = "NewName".to_string();
    app.picker_name_cursor = 7;

    handle_key_event(&mut app, create_key_event(KeyCode::Esc));

    assert_eq!(app.picker_mode, PickerMode::Selecting);
    assert!(app.show_request_picker);
    assert!(!app.should_quit);
}

#[test]
fn test_picker_renaming_text_input() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Renaming;
    app.picker_name_input = "Old".to_string();
    app.picker_name_cursor = 3;

    handle_key_event(&mut app, create_key_event(KeyCode::Char('!')));

    assert_eq!(app.picker_name_input, "Old!");
    assert_eq!(app.picker_name_cursor, 4);
}

#[test]
fn test_picker_renaming_backspace() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Renaming;
    app.picker_name_input = "Test".to_string();
    app.picker_name_cursor = 4;

    handle_key_event(&mut app, create_key_event(KeyCode::Backspace));

    assert_eq!(app.picker_name_input, "Tes");
    assert_eq!(app.picker_name_cursor, 3);
}

#[test]
fn test_picker_renaming_cursor_movement() {
    let mut app = App::new();
    app.show_request_picker = true;
    app.picker_mode = PickerMode::Renaming;
    app.picker_name_input = "Test".to_string();
    app.picker_name_cursor = 4;

    handle_key_event(&mut app, create_key_event(KeyCode::Left));
    assert_eq!(app.picker_name_cursor, 3);

    handle_key_event(&mut app, create_key_event(KeyCode::Right));
    assert_eq!(app.picker_name_cursor, 4);
}
