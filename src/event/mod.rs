use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, AppFocus, METHODS};

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
                AppFocus::UrlInput => AppFocus::Response,
                AppFocus::Response => AppFocus::MethodSelector,
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

#[cfg(test)]
mod tests;
