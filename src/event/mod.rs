use crate::app::{App, AppFocus, METHODS};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    // Global keybindings
    match key.code {
        KeyCode::Esc => {
            app.should_quit = true;
            return;
        }
        KeyCode::Tab => {
            app.focus = match app.focus {
                AppFocus::MethodSelector => AppFocus::UrlInput,
                AppFocus::UrlInput => AppFocus::HeadersInput,
                AppFocus::HeadersInput => AppFocus::BodyInput,
                AppFocus::BodyInput => AppFocus::Response,
                AppFocus::Response => AppFocus::MethodSelector,
            };
            return;
        }
        KeyCode::BackTab => {
            app.focus = match app.focus {
                AppFocus::MethodSelector => AppFocus::Response,
                AppFocus::UrlInput => AppFocus::MethodSelector,
                AppFocus::HeadersInput => AppFocus::UrlInput,
                AppFocus::BodyInput => AppFocus::HeadersInput,
                AppFocus::Response => AppFocus::BodyInput,
            };
            return;
        }
        _ => {}
    }

    // Context-specific keybindings
    match app.focus {
        AppFocus::MethodSelector => {
            if !app.loading {
                match key.code {
                    KeyCode::Up => {
                        if app.method_index > 0 {
                            app.method_index -= 1;
                        } else {
                            app.method_index = METHODS.len() - 1;
                        }
                        app.http_method = METHODS[app.method_index].to_string();
                    }
                    KeyCode::Down => {
                        if app.method_index < METHODS.len() - 1 {
                            app.method_index += 1;
                        } else {
                            app.method_index = 0;
                        }
                        app.http_method = METHODS[app.method_index].to_string();
                    }
                    KeyCode::Enter => {
                        app.send_request();
                    }
                    _ => {}
                }
            }
        }
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
        AppFocus::HeadersInput => {
            if !app.loading {
                match key.code {
                    KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Insert 2 spaces at cursor (indent)
                        app.handle_multiline_char(' ', true);
                        app.handle_multiline_char(' ', true);
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.send_request();
                    }
                    KeyCode::Char(c) => {
                        app.handle_multiline_char(c, true);
                    }
                    KeyCode::Backspace => {
                        app.handle_multiline_backspace(true);
                        app.ensure_headers_cursor_visible(3);
                    }
                    KeyCode::Enter
                        if key.modifiers.contains(KeyModifiers::CONTROL)
                            || key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        app.send_request();
                    }
                    KeyCode::Enter => {
                        app.handle_multiline_enter(true);
                        app.ensure_headers_cursor_visible(3);
                    }
                    KeyCode::Up => {
                        app.handle_multiline_up(true);
                        app.ensure_headers_cursor_visible(3);
                    }
                    KeyCode::Down => {
                        app.handle_multiline_down(true);
                        app.ensure_headers_cursor_visible(3);
                    }
                    KeyCode::Left => {
                        app.handle_multiline_left(true);
                    }
                    KeyCode::Right => {
                        app.handle_multiline_right(true);
                    }
                    _ => {}
                }
            }
        }
        AppFocus::BodyInput => {
            if !app.loading {
                match key.code {
                    KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Insert 2 spaces at cursor (indent)
                        app.handle_multiline_char(' ', false);
                        app.handle_multiline_char(' ', false);
                    }
                    KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.format_body_json();
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.send_request();
                    }
                    KeyCode::Char(c) => {
                        app.handle_multiline_char(c, false);
                    }
                    KeyCode::Backspace => {
                        app.handle_multiline_backspace(false);
                        app.ensure_body_cursor_visible(6);
                    }
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.handle_multiline_enter(false);
                        app.ensure_body_cursor_visible(6);
                    }
                    KeyCode::Enter
                        if key.modifiers.contains(KeyModifiers::CONTROL)
                            || key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        app.send_request();
                    }
                    KeyCode::Enter => {
                        app.handle_multiline_enter(false);
                        app.ensure_body_cursor_visible(6);
                    }
                    KeyCode::Up => {
                        app.handle_multiline_up(false);
                        app.ensure_body_cursor_visible(6);
                    }
                    KeyCode::Down => {
                        app.handle_multiline_down(false);
                        app.ensure_body_cursor_visible(6);
                    }
                    KeyCode::Left => {
                        app.handle_multiline_left(false);
                    }
                    KeyCode::Right => {
                        app.handle_multiline_right(false);
                    }
                    _ => {}
                }
            }
        }
        AppFocus::Response => match key.code {
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
        },
    }
}

#[cfg(test)]
mod tests;
