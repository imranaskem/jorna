pub const METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppFocus {
    MethodSelector,
    UrlInput,
    Response,
}

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
}

impl App {
    pub fn new() -> Self {
        let default_url = "https://pokeapi.co/api/v2/pokemon/snorlax".to_string();
        let cursor_pos = default_url.len();

        Self {
            url_input: default_url,
            cursor_position: cursor_pos,
            response: "Response will appear here...".to_string(),
            response_scroll: 0,
            loading: false,
            focus: AppFocus::MethodSelector,
            should_quit: false,
            http_method: "GET".to_string(),
            method_index: 0,
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

        // Simple synchronous request with method selection
        let client = reqwest::blocking::Client::new();
        let response_result = match self.http_method.as_str() {
            "GET" => client.get(&url).send(),
            "POST" => client.post(&url).send(),
            "PUT" => client.put(&url).send(),
            "DELETE" => client.delete(&url).send(),
            "PATCH" => client.patch(&url).send(),
            "HEAD" => client.head(&url).send(),
            "OPTIONS" => client.request(reqwest::Method::OPTIONS, &url).send(),
            _ => {
                self.response = "Error: Invalid HTTP method".to_string();
                self.loading = false;
                return;
            }
        };

        let response_text = match response_result {
            Ok(response) => {
                let status = response.status();
                let headers = format!("Status: {}\n\n", status);
                match response.text() {
                    Ok(body) => {
                        // Try to parse and pretty-print JSON
                        let formatted_body = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            serde_json::to_string_pretty(&json).unwrap_or(body)
                        } else {
                            body
                        };
                        format!("{}{}", headers, formatted_body)
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
}

#[cfg(test)]
mod tests;
