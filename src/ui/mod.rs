use crate::app::{App, AppFocus};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
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

    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Url")
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
                "↑↓: Change Method | Enter: Send | Tab/Shift+Tab: Switch Focus | Esc: Quit"
            }
            AppFocus::UrlInput => "Enter: Send | Tab/Shift+Tab: Switch Focus | ←→: Move Cursor | Esc: Quit",
            AppFocus::HeadersInput => "Shift+Enter: New line | Enter: Send | Tab/Shift+Tab: Switch Focus | Esc: Quit",
            AppFocus::BodyInput => "Ctrl+T: Indent | Ctrl+F: Format | Ctrl+S: Send | Tab/Shift+Tab: Switch Focus | Esc: Quit",
            AppFocus::Response => "↑↓: Scroll | Tab/Shift+Tab: Switch Focus | Esc: Quit",
        }
    };
    let instructions_widget =
        Paragraph::new(instructions).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(instructions_widget, chunks[5]);
}

#[cfg(test)]
mod tests;
