use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, AppFocus};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Input area (method + URL)
            Constraint::Length(1),  // Instructions
            Constraint::Min(10),    // Response area
        ])
        .split(frame.area());

    // Split input area horizontally for method selector and URL input
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12),  // Method selector
            Constraint::Min(20),     // URL input
        ])
        .split(chunks[0]);

    // Method selector
    let method_text = format!(" {} ", app.http_method);
    let method_widget = Paragraph::new(method_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("METHOD")
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
                .title("URL")
                .border_style(if app.focus == AppFocus::UrlInput {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(input_widget, input_chunks[1]);

    // Instructions
    let instructions = if app.loading {
        "Loading..."
    } else {
        match app.focus {
            AppFocus::MethodSelector => "↑↓: Change Method | Enter: Send | Tab: Next Focus | Esc: Quit",
            AppFocus::UrlInput => "Enter: Send | Tab: Next Focus | ←→: Move Cursor | Esc: Quit",
            AppFocus::Response => "↑↓: Scroll Response | Tab: Next Focus | Esc: Quit",
        }
    };
    let instructions_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(instructions_widget, chunks[1]);

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

    frame.render_widget(response_widget, chunks[2]);
}

#[cfg(test)]
mod tests;
