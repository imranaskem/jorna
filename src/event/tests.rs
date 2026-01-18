use super::*;
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
