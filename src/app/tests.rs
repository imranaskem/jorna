use super::*;

#[test]
fn test_app_initialization() {
    let app = App::new();

    assert_eq!(app.url_input, "https://pokeapi.co/api/v2/pokemon/snorlax");
    assert_eq!(app.cursor_position, app.url_input.len());
    assert_eq!(app.response, "Response will appear here...");
    assert_eq!(app.response_scroll, 0);
    assert_eq!(app.loading, false);
    assert_eq!(app.focus, AppFocus::MethodSelector);
    assert_eq!(app.should_quit, false);
    assert_eq!(app.http_method, "GET");
    assert_eq!(app.method_index, 0);
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
    assert_eq!(app.loading, false);
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
