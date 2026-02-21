use crate::app::{App, AppFocus, PickerEntry, PickerMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Input area (method + URL)
            Constraint::Length(5), // Headers input
            Constraint::Length(8), // Body input
            Constraint::Length(1), // Status line
            Constraint::Min(10),   // Response area
            Constraint::Length(1), // Instructions
        ])
        .split(frame.area());

    // Split input area horizontally for method selector and URL input
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12), // Method selector
            Constraint::Min(20),    // URL input
        ])
        .split(chunks[0]);

    // Method selector
    let method_text = format!(" {} ", app.http_method);
    let method_widget = Paragraph::new(method_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Method")
                .border_style(if app.focus == AppFocus::MethodSelector {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(method_widget, input_chunks[0]);

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

    let url_title = format!("Url [{}]", app.request_name);
    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(url_title)
                .border_style(if app.focus == AppFocus::UrlInput {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(input_widget, input_chunks[1]);

    // Headers input
    let headers_text = if app.focus == AppFocus::HeadersInput {
        // Show cursor when focused
        let mut lines_with_cursor = Vec::new();
        for (i, line) in app.headers_input.iter().enumerate() {
            if i == app.headers_cursor_line {
                let before = &line[..app.headers_cursor_col.min(line.len())];
                let after = &line[app.headers_cursor_col.min(line.len())..];
                lines_with_cursor.push(Line::from(vec![
                    Span::raw(before),
                    Span::styled("█", Style::default().fg(Color::Cyan)),
                    Span::raw(after),
                ]));
            } else {
                lines_with_cursor.push(Line::from(line.as_str()));
            }
        }
        lines_with_cursor
    } else {
        app.headers_input
            .iter()
            .map(|line| Line::from(line.as_str()))
            .collect()
    };

    let headers_widget = Paragraph::new(headers_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Headers (Key: Value per line)")
                .border_style(if app.focus == AppFocus::HeadersInput {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        )
        .scroll((app.headers_scroll, 0));
    frame.render_widget(headers_widget, chunks[1]);

    // Body input
    let body_text = if app.focus == AppFocus::BodyInput {
        // Show cursor when focused
        let mut lines_with_cursor = Vec::new();
        for (i, line) in app.body_input.iter().enumerate() {
            if i == app.body_cursor_line {
                let before = &line[..app.body_cursor_col.min(line.len())];
                let after = &line[app.body_cursor_col.min(line.len())..];
                lines_with_cursor.push(Line::from(vec![
                    Span::raw(before),
                    Span::styled("█", Style::default().fg(Color::Cyan)),
                    Span::raw(after),
                ]));
            } else {
                lines_with_cursor.push(Line::from(line.as_str()));
            }
        }
        lines_with_cursor
    } else {
        app.body_input
            .iter()
            .map(|line| Line::from(line.as_str()))
            .collect()
    };

    let body_widget = Paragraph::new(body_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Body (JSON)")
                .border_style(if app.focus == AppFocus::BodyInput {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        )
        .scroll((app.body_scroll, 0));
    frame.render_widget(body_widget, chunks[2]);

    // Status line
    let status_text = if app.loading {
        "Loading...".to_string()
    } else if let Some(status_code) = app.status_code {
        let mut parts = vec![format!("Status: {}", status_code)];
        if let Some(duration) = app.response_time {
            let ms = duration.as_millis();
            if ms >= 1000 {
                parts.push(format!("Time: {:.2}s", duration.as_secs_f64()));
            } else {
                parts.push(format!("Time: {}ms", ms));
            }
        }
        if let Some(size) = app.response_size {
            if size >= 1024 * 1024 {
                parts.push(format!("Size: {:.2}MB", size as f64 / (1024.0 * 1024.0)));
            } else if size >= 1024 {
                parts.push(format!("Size: {:.2}KB", size as f64 / 1024.0));
            } else {
                parts.push(format!("Size: {}B", size));
            }
        }
        parts.join(" │ ")
    } else {
        String::new()
    };
    let status_widget = Paragraph::new(status_text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(status_widget, chunks[3]);

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

    frame.render_widget(response_widget, chunks[4]);

    // Instructions
    let instructions = if app.loading {
        "Loading..."
    } else {
        match app.focus {
            AppFocus::MethodSelector => {
                "↑↓: Change Method | Enter: Send | Tab/Shift+Tab: Switch Focus | Ctrl+P: Requests | Esc: Quit"
            }
            AppFocus::UrlInput => "Enter: Send | Tab/Shift+Tab: Switch Focus | ←→: Move Cursor | Ctrl+P: Requests | Esc: Quit",
            AppFocus::HeadersInput => "Ctrl+T: Indent | Ctrl+S: Send | Tab/Shift+Tab: Switch Focus | Ctrl+P: Requests | Esc: Quit",
            AppFocus::BodyInput => "Ctrl+T: Indent | Ctrl+F: Format | Ctrl+S: Send | Tab/Shift+Tab: Switch Focus | Ctrl+P: Requests | Esc: Quit",
            AppFocus::Response => "↑↓: Scroll | Tab/Shift+Tab: Switch Focus | Ctrl+P: Requests | Esc: Quit",
        }
    };
    let instructions_widget =
        Paragraph::new(instructions).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(instructions_widget, chunks[5]);

    // Picker overlay
    if app.show_request_picker {
        render_picker_overlay(frame, app);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_picker_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 50, frame.area());

    frame.render_widget(Clear, area);

    // Build title showing folder breadcrumb
    let title = if app.picker_current_folder.is_empty() {
        "Requests".to_string()
    } else {
        format!(
            "Requests > {}",
            app.picker_current_folder.replace('/', " > ")
        )
    };

    // Split area for list + instructions
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // List area
            Constraint::Length(2), // Instructions
        ])
        .split(area);

    // Build list lines
    let mut lines: Vec<Line> = Vec::new();

    if app.picker_entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (empty folder)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, entry) in app.picker_entries.iter().enumerate() {
            let is_selected = i == app.picker_selected;

            let (display, is_active) = match entry {
                PickerEntry::Folder { name } => (format!("  {}/", name), false),
                PickerEntry::Request { name, path } => {
                    let marker = if path == &app.request_path {
                        "* "
                    } else {
                        "  "
                    };
                    (format!("{}{}", marker, name), path == &app.request_path)
                }
            };

            let style = if is_selected {
                Style::default().bg(Color::Cyan).fg(Color::Black)
            } else if is_active {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            lines.push(Line::from(Span::styled(display, style)));
        }
    }

    let list_widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(list_widget, inner_chunks[0]);

    // Instructions line
    let instructions_text = match app.picker_mode {
        PickerMode::Selecting => {
            "↑↓: Navigate | Enter: Open | Backspace: Back | N: New | R: Rename | Ctrl+D: Delete | Esc: Close"
        }
        PickerMode::Naming => "Enter: Create | Esc: Cancel",
        PickerMode::Renaming => "Enter: Rename | Esc: Cancel",
    };

    let instructions_line = match app.picker_mode {
        PickerMode::Naming | PickerMode::Renaming => {
            let label = if app.picker_mode == PickerMode::Naming {
                "Name: "
            } else {
                "Rename: "
            };
            let before = &app.picker_name_input[..app.picker_name_cursor];
            let after = &app.picker_name_input[app.picker_name_cursor..];
            Line::from(vec![
                Span::styled(label, Style::default().fg(Color::DarkGray)),
                Span::raw(before),
                Span::styled("█", Style::default().fg(Color::Cyan)),
                Span::raw(after),
                Span::styled(
                    format!("  ({})", instructions_text),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        }
        _ => Line::from(Span::styled(
            instructions_text,
            Style::default().fg(Color::DarkGray),
        )),
    };

    let instructions_widget = Paragraph::new(instructions_line).wrap(Wrap { trim: false });
    frame.render_widget(instructions_widget, inner_chunks[1]);
}

#[cfg(test)]
mod tests;
