use std::time::{Duration, Instant};

pub const METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppFocus {
    MethodSelector,
    UrlInput,
    HeadersInput,
    BodyInput,
    Response,
}

#[derive(Clone)]
pub struct App {
    pub url_input: String,
    pub cursor_position: usize,
    pub response: String,
    pub response_scroll: u16,
    pub loading: bool,
    pub focus: AppFocus,
    pub should_quit: bool,
    pub http_method: String,
    pub method_index: usize,
    pub headers_input: Vec<String>,
    pub headers_cursor_line: usize,
    pub headers_cursor_col: usize,
    pub headers_scroll: u16,
    pub body_input: Vec<String>,
    pub body_cursor_line: usize,
    pub body_cursor_col: usize,
    pub body_scroll: u16,
    pub response_time: Option<Duration>,
    pub status_code: Option<u16>,
    pub response_size: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        let default_url = "https://pokeapi.co/api/v2/pokemon/snorlax".to_string();
        let cursor_pos = default_url.len();

        Self {
            url_input: default_url,
            cursor_position: cursor_pos,
            response: "{}".to_string(),
            response_scroll: 0,
            loading: false,
            focus: AppFocus::MethodSelector,
            should_quit: false,
            http_method: "GET".to_string(),
            method_index: 0,
            headers_input: vec![String::new()],
            headers_cursor_line: 0,
            headers_cursor_col: 0,
            headers_scroll: 0,
            body_input: vec![String::new()],
            body_cursor_line: 0,
            body_cursor_col: 0,
            body_scroll: 0,
            response_time: None,
            status_code: None,
            response_size: None,
        }
    }

    pub fn send_request(&mut self) {
        if self.url_input.is_empty() {
            self.response = "Error: URL cannot be empty".to_string();
            return;
        }

        let url = self.url_input.clone();
        self.loading = true;
        self.response = "Loading...".to_string();
        self.response_scroll = 0;
        self.response_time = None;
        self.status_code = None;
        self.response_size = None;

        // Parse headers from headers_input
        let headers: Vec<(String, String)> = self
            .headers_input
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, ": ").collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            })
            .collect();

        // Get body text from body_input
        let body_text = self.body_input.join("\n").trim().to_string();

        // Validate JSON if body is not empty
        if !body_text.is_empty() {
            if let Err(e) = serde_json::from_str::<serde_json::Value>(&body_text) {
                self.response = format!("Error: Invalid JSON in body: {}", e);
                self.loading = false;
                return;
            }
        }

        // Build request with method
        let client = reqwest::blocking::Client::new();
        let mut request = match self.http_method.as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url),
            "HEAD" => client.head(&url),
            "OPTIONS" => client.request(reqwest::Method::OPTIONS, &url),
            _ => {
                self.response = "Error: Invalid HTTP method".to_string();
                self.loading = false;
                return;
            }
        };

        // Add headers
        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Add body if present
        if !body_text.is_empty() {
            request = request.body(body_text);
        }

        // Send request
        let start = Instant::now();
        let response_result = request.send();
        let elapsed = start.elapsed();

        let response_text = match response_result {
            Ok(response) => {
                let status = response.status();
                self.status_code = Some(status.as_u16());
                self.response_time = Some(elapsed);
                match response.text() {
                    Ok(body) => {
                        self.response_size = Some(body.len());
                        // Try to parse and pretty-print JSON
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            serde_json::to_string_pretty(&json).unwrap_or(body)
                        } else {
                            body
                        }
                    }
                    Err(e) => format!("Error reading response: {}", e),
                }
            }
            Err(e) => format!("Request failed: {}", e),
        };

        self.response = response_text;
        self.loading = false;
    }

    pub fn handle_input_char(&mut self, c: char) {
        self.url_input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.url_input.remove(self.cursor_position);
        }
    }

    pub fn handle_delete(&mut self) {
        if self.cursor_position < self.url_input.len() {
            self.url_input.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.url_input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.url_input.len();
    }

    // Multi-line text input helpers
    pub fn handle_multiline_char(&mut self, c: char, is_headers: bool) {
        let (lines, cursor_line, cursor_col) = if is_headers {
            (
                &mut self.headers_input,
                &mut self.headers_cursor_line,
                &mut self.headers_cursor_col,
            )
        } else {
            (
                &mut self.body_input,
                &mut self.body_cursor_line,
                &mut self.body_cursor_col,
            )
        };

        if *cursor_line >= lines.len() {
            lines.push(String::new());
            *cursor_line = lines.len() - 1;
        }

        lines[*cursor_line].insert(*cursor_col, c);
        *cursor_col += 1;
    }

    pub fn handle_multiline_backspace(&mut self, is_headers: bool) {
        let (lines, cursor_line, cursor_col) = if is_headers {
            (
                &mut self.headers_input,
                &mut self.headers_cursor_line,
                &mut self.headers_cursor_col,
            )
        } else {
            (
                &mut self.body_input,
                &mut self.body_cursor_line,
                &mut self.body_cursor_col,
            )
        };

        if *cursor_col > 0 {
            *cursor_col -= 1;
            lines[*cursor_line].remove(*cursor_col);
        } else if *cursor_line > 0 {
            let current_line = lines.remove(*cursor_line);
            *cursor_line -= 1;
            *cursor_col = lines[*cursor_line].len();
            lines[*cursor_line].push_str(&current_line);
        }
    }

    pub fn handle_multiline_enter(&mut self, is_headers: bool) {
        let (lines, cursor_line, cursor_col) = if is_headers {
            (
                &mut self.headers_input,
                &mut self.headers_cursor_line,
                &mut self.headers_cursor_col,
            )
        } else {
            (
                &mut self.body_input,
                &mut self.body_cursor_line,
                &mut self.body_cursor_col,
            )
        };

        let rest = lines[*cursor_line].split_off(*cursor_col);
        *cursor_line += 1;
        lines.insert(*cursor_line, rest);
        *cursor_col = 0;
    }

    pub fn handle_multiline_up(&mut self, is_headers: bool) {
        let (cursor_line, cursor_col, lines) = if is_headers {
            (
                &mut self.headers_cursor_line,
                &mut self.headers_cursor_col,
                &self.headers_input,
            )
        } else {
            (
                &mut self.body_cursor_line,
                &mut self.body_cursor_col,
                &self.body_input,
            )
        };

        if *cursor_line > 0 {
            *cursor_line -= 1;
            *cursor_col = (*cursor_col).min(lines[*cursor_line].len());
        }
    }

    pub fn handle_multiline_down(&mut self, is_headers: bool) {
        let (cursor_line, cursor_col, lines) = if is_headers {
            (
                &mut self.headers_cursor_line,
                &mut self.headers_cursor_col,
                &self.headers_input,
            )
        } else {
            (
                &mut self.body_cursor_line,
                &mut self.body_cursor_col,
                &self.body_input,
            )
        };

        if *cursor_line + 1 < lines.len() {
            *cursor_line += 1;
            *cursor_col = (*cursor_col).min(lines[*cursor_line].len());
        }
    }

    pub fn handle_multiline_left(&mut self, is_headers: bool) {
        let cursor_col = if is_headers {
            &mut self.headers_cursor_col
        } else {
            &mut self.body_cursor_col
        };

        if *cursor_col > 0 {
            *cursor_col -= 1;
        }
    }

    pub fn handle_multiline_right(&mut self, is_headers: bool) {
        let (cursor_line, cursor_col, lines) = if is_headers {
            (
                &self.headers_cursor_line,
                &mut self.headers_cursor_col,
                &self.headers_input,
            )
        } else {
            (
                &self.body_cursor_line,
                &mut self.body_cursor_col,
                &self.body_input,
            )
        };

        if *cursor_col < lines[*cursor_line].len() {
            *cursor_col += 1;
        }
    }

    pub fn ensure_body_cursor_visible(&mut self, visible_lines: usize) {
        if visible_lines == 0 {
            return;
        }
        let scroll = self.body_scroll as usize;
        if self.body_cursor_line < scroll {
            self.body_scroll = self.body_cursor_line as u16;
        } else if self.body_cursor_line >= scroll + visible_lines {
            self.body_scroll = (self.body_cursor_line - visible_lines + 1) as u16;
        }
    }

    pub fn ensure_headers_cursor_visible(&mut self, visible_lines: usize) {
        if visible_lines == 0 {
            return;
        }
        let scroll = self.headers_scroll as usize;
        if self.headers_cursor_line < scroll {
            self.headers_scroll = self.headers_cursor_line as u16;
        } else if self.headers_cursor_line >= scroll + visible_lines {
            self.headers_scroll = (self.headers_cursor_line - visible_lines + 1) as u16;
        }
    }

    pub fn format_body_json(&mut self) {
        let body_text = self.body_input.join("\n");
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
            if let Ok(formatted) = serde_json::to_string_pretty(&json) {
                self.body_input = formatted.lines().map(String::from).collect();
                // Reset cursor to start
                self.body_cursor_line = 0;
                self.body_cursor_col = 0;
            }
        }
        // If invalid JSON, do nothing silently
    }
}

#[cfg(test)]
mod tests;
