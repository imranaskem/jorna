use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub const METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppFocus {
    MethodSelector,
    UrlInput,
    HeadersInput,
    BodyInput,
    Response,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PickerEntry {
    Folder { name: String },
    Request { name: String, path: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PickerMode {
    #[default]
    Selecting,
    Naming,
    Renaming,
}

mod option_duration_millis {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(d) => serializer.serialize_some(&d.as_millis()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<u64> = Option::deserialize(deserializer)?;
        Ok(opt.map(Duration::from_millis))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct App {
    pub url_input: String,
    pub cursor_position: usize,
    pub response: String,
    pub response_scroll: u16,
    #[serde(skip)]
    pub loading: bool,
    pub focus: AppFocus,
    #[serde(skip)]
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
    #[serde(with = "option_duration_millis")]
    pub response_time: Option<Duration>,
    pub status_code: Option<u16>,
    pub response_size: Option<usize>,

    // Request identity (not serialized to request file — derived from filesystem)
    #[serde(skip)]
    pub request_name: String,
    #[serde(skip)]
    pub request_path: String,

    // Picker overlay state
    #[serde(skip)]
    pub show_request_picker: bool,
    #[serde(skip)]
    pub picker_entries: Vec<PickerEntry>,
    #[serde(skip)]
    pub picker_selected: usize,
    #[serde(skip)]
    pub picker_current_folder: String,
    #[serde(skip)]
    pub picker_mode: PickerMode,
    #[serde(skip)]
    pub picker_name_input: String,
    #[serde(skip)]
    pub picker_name_cursor: usize,
    #[serde(skip)]
    pub picker_rename_path: String,
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
            request_name: "Default".to_string(),
            request_path: "Default".to_string(),
            show_request_picker: false,
            picker_entries: Vec::new(),
            picker_selected: 0,
            picker_current_folder: String::new(),
            picker_mode: PickerMode::Selecting,
            picker_name_input: String::new(),
            picker_name_cursor: 0,
            picker_rename_path: String::new(),
        }
    }

    // --- Persistence ---

    pub fn jorna_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".jorna"))
    }

    pub fn requests_dir() -> Option<PathBuf> {
        Self::jorna_dir().map(|d| d.join("requests"))
    }

    pub fn state_file_path() -> Option<PathBuf> {
        Self::jorna_dir().map(|d| d.join("state.json"))
    }

    pub fn save_request(&self) {
        let Some(requests_dir) = Self::requests_dir() else {
            return;
        };
        let file_path = requests_dir.join(format!("{}.json", self.request_path));
        let Ok(json) = serde_json::to_string_pretty(self) else {
            return;
        };
        if let Some(parent) = file_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(file_path, json);
    }

    pub fn load_request(&mut self, path: &str) {
        let Some(requests_dir) = Self::requests_dir() else {
            return;
        };
        let file_path = requests_dir.join(format!("{}.json", path));
        let Ok(data) = std::fs::read_to_string(file_path) else {
            return;
        };
        let Ok(loaded) = serde_json::from_str::<App>(&data) else {
            return;
        };

        self.url_input = loaded.url_input;
        self.cursor_position = loaded.cursor_position;
        self.response = loaded.response;
        self.response_scroll = loaded.response_scroll;
        self.focus = loaded.focus;
        self.http_method = loaded.http_method;
        self.method_index = loaded.method_index;
        self.headers_input = loaded.headers_input;
        self.headers_cursor_line = loaded.headers_cursor_line;
        self.headers_cursor_col = loaded.headers_cursor_col;
        self.headers_scroll = loaded.headers_scroll;
        self.body_input = loaded.body_input;
        self.body_cursor_line = loaded.body_cursor_line;
        self.body_cursor_col = loaded.body_cursor_col;
        self.body_scroll = loaded.body_scroll;
        self.response_time = loaded.response_time;
        self.status_code = loaded.status_code;
        self.response_size = loaded.response_size;

        // Derive name from path
        self.request_path = path.to_string();
        self.request_name = path.rsplit('/').next().unwrap_or(path).to_string();
    }

    pub fn list_folder(folder: &str) -> Vec<PickerEntry> {
        let Some(requests_dir) = Self::requests_dir() else {
            return Vec::new();
        };
        let dir = if folder.is_empty() {
            requests_dir
        } else {
            requests_dir.join(folder)
        };
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return Vec::new();
        };

        let mut folders = Vec::new();
        let mut requests = Vec::new();

        for entry in entries.flatten() {
            let file_type = entry.file_type();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if let Ok(ft) = file_type {
                if ft.is_dir() {
                    folders.push(PickerEntry::Folder { name: file_name });
                } else if file_name.ends_with(".json") {
                    let name = file_name.trim_end_matches(".json").to_string();
                    let path = if folder.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", folder, name)
                    };
                    requests.push(PickerEntry::Request { name, path });
                }
            }
        }

        folders.sort_by(|a, b| {
            let a_name = match a {
                PickerEntry::Folder { name } => name,
                _ => unreachable!(),
            };
            let b_name = match b {
                PickerEntry::Folder { name } => name,
                _ => unreachable!(),
            };
            a_name.to_lowercase().cmp(&b_name.to_lowercase())
        });

        requests.sort_by(|a, b| {
            let a_name = match a {
                PickerEntry::Request { name, .. } => name,
                _ => unreachable!(),
            };
            let b_name = match b {
                PickerEntry::Request { name, .. } => name,
                _ => unreachable!(),
            };
            a_name.to_lowercase().cmp(&b_name.to_lowercase())
        });

        folders.extend(requests);
        folders
    }

    pub fn delete_request(path: &str) {
        let Some(requests_dir) = Self::requests_dir() else {
            return;
        };
        let file_path = requests_dir.join(format!("{}.json", path));
        let _ = std::fs::remove_file(&file_path);

        // Remove empty parent directories
        if let Some(parent) = file_path.parent() {
            if parent != requests_dir {
                // Only remove if it's empty
                if let Ok(mut entries) = std::fs::read_dir(parent) {
                    if entries.next().is_none() {
                        let _ = std::fs::remove_dir(parent);
                    }
                }
            }
        }
    }

    pub fn save_active_path(&self) {
        let Some(path) = Self::state_file_path() else {
            return;
        };
        let json = format!(
            "{{\"active_request_path\":{}}}",
            serde_json::to_string(&self.request_path).unwrap_or_default()
        );
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, json);
    }

    pub fn load_active_path() -> Option<String> {
        let path = Self::state_file_path()?;
        let data = std::fs::read_to_string(path).ok()?;
        let v: serde_json::Value = serde_json::from_str(&data).ok()?;
        v.get("active_request_path")?.as_str().map(String::from)
    }

    /// Migrate old state.json (contains url_input etc.) to new format
    pub fn migrate_old_state() -> bool {
        let Some(state_path) = Self::state_file_path() else {
            return false;
        };
        let Ok(data) = std::fs::read_to_string(&state_path) else {
            return false;
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&data) else {
            return false;
        };

        // If old format (has url_input key), migrate
        if v.get("url_input").is_some() {
            // Parse as old App
            if let Ok(old_app) = serde_json::from_str::<App>(&data) {
                let mut app = old_app;
                app.request_name = "Default".to_string();
                app.request_path = "Default".to_string();
                app.save_request();
                app.save_active_path();
                return true;
            }
        }
        false
    }

    /// Initialize app from saved state, handling migration
    pub fn initialize() -> Self {
        // Try migration first
        Self::migrate_old_state();

        let mut app = App::new();

        if let Some(active_path) = Self::load_active_path() {
            app.load_request(&active_path);
        } else {
            // No state at all — create default request
            app.save_request();
            app.save_active_path();
        }

        app
    }

    // --- Picker methods ---

    pub fn open_picker(&mut self) {
        self.save_request();
        self.show_request_picker = true;
        self.picker_mode = PickerMode::Selecting;

        // Navigate to parent folder of current request
        self.picker_current_folder = if let Some(pos) = self.request_path.rfind('/') {
            self.request_path[..pos].to_string()
        } else {
            String::new()
        };

        self.picker_entries = Self::list_folder(&self.picker_current_folder);

        // Select current request in list
        self.picker_selected = self
            .picker_entries
            .iter()
            .position(
                |e| matches!(e, PickerEntry::Request { path, .. } if path == &self.request_path),
            )
            .unwrap_or(0);
    }

    pub fn close_picker(&mut self) {
        self.show_request_picker = false;
        self.picker_mode = PickerMode::Selecting;
    }

    pub fn picker_enter(&mut self) {
        if self.picker_entries.is_empty() {
            return;
        }

        let selected = self.picker_selected.min(self.picker_entries.len() - 1);
        let entry = self.picker_entries[selected].clone();

        match entry {
            PickerEntry::Folder { name } => {
                // Navigate into folder
                self.picker_current_folder = if self.picker_current_folder.is_empty() {
                    name
                } else {
                    format!("{}/{}", self.picker_current_folder, name)
                };
                self.picker_entries = Self::list_folder(&self.picker_current_folder);
                self.picker_selected = 0;
            }
            PickerEntry::Request { path, .. } => {
                self.save_request();
                self.load_request(&path);
                self.save_active_path();
                self.close_picker();
            }
        }
    }

    pub fn picker_go_back(&mut self) {
        if self.picker_current_folder.is_empty() {
            self.close_picker();
            return;
        }

        self.picker_current_folder = if let Some(pos) = self.picker_current_folder.rfind('/') {
            self.picker_current_folder[..pos].to_string()
        } else {
            String::new()
        };

        self.picker_entries = Self::list_folder(&self.picker_current_folder);
        self.picker_selected = 0;
    }

    pub fn picker_create_request(&mut self, name_input: String) {
        let name_input = name_input.trim().to_string();
        if name_input.is_empty() {
            return;
        }

        // Resolve path: if picker is in a subfolder, prepend it
        let path = if self.picker_current_folder.is_empty() {
            name_input
        } else {
            format!("{}/{}", self.picker_current_folder, name_input)
        };

        // Save current request first
        self.save_request();

        // Create a blank request
        let default_url = String::new();
        self.url_input = default_url;
        self.cursor_position = 0;
        self.response = "{}".to_string();
        self.response_scroll = 0;
        self.focus = AppFocus::UrlInput;
        self.http_method = "GET".to_string();
        self.method_index = 0;
        self.headers_input = vec![String::new()];
        self.headers_cursor_line = 0;
        self.headers_cursor_col = 0;
        self.headers_scroll = 0;
        self.body_input = vec![String::new()];
        self.body_cursor_line = 0;
        self.body_cursor_col = 0;
        self.body_scroll = 0;
        self.response_time = None;
        self.status_code = None;
        self.response_size = None;

        self.request_path = path;
        self.request_name = self
            .request_path
            .rsplit('/')
            .next()
            .unwrap_or(&self.request_path)
            .to_string();

        self.save_request();
        self.save_active_path();
        self.close_picker();
    }

    pub fn picker_delete_selected(&mut self) {
        if self.picker_entries.is_empty() {
            return;
        }

        let selected = self.picker_selected.min(self.picker_entries.len() - 1);
        let entry = self.picker_entries[selected].clone();

        if let PickerEntry::Request { path, .. } = entry {
            // Count total requests to prevent deleting the last one
            let total = self.count_all_requests();
            if total <= 1 {
                return;
            }

            // If deleting the active request, switch to another one first
            if path == self.request_path {
                // Find another request to switch to
                if let Some(other) = self.find_another_request(&path) {
                    self.load_request(&other);
                    self.save_active_path();
                }
            }

            Self::delete_request(&path);
            self.picker_entries = Self::list_folder(&self.picker_current_folder);
            if self.picker_selected >= self.picker_entries.len() && !self.picker_entries.is_empty()
            {
                self.picker_selected = self.picker_entries.len() - 1;
            }
        }
    }

    fn count_all_requests(&self) -> usize {
        Self::count_requests_in_folder("")
    }

    fn count_requests_in_folder(folder: &str) -> usize {
        let entries = Self::list_folder(folder);
        let mut count = 0;
        for entry in &entries {
            match entry {
                PickerEntry::Request { .. } => count += 1,
                PickerEntry::Folder { name } => {
                    let subfolder = if folder.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", folder, name)
                    };
                    count += Self::count_requests_in_folder(&subfolder);
                }
            }
        }
        count
    }

    fn find_another_request(&self, exclude_path: &str) -> Option<String> {
        Self::find_request_in_folder("", exclude_path)
    }

    fn find_request_in_folder(folder: &str, exclude_path: &str) -> Option<String> {
        let entries = Self::list_folder(folder);
        for entry in &entries {
            match entry {
                PickerEntry::Request { path, .. } if path != exclude_path => {
                    return Some(path.clone());
                }
                PickerEntry::Folder { name } => {
                    let subfolder = if folder.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", folder, name)
                    };
                    if let Some(found) = Self::find_request_in_folder(&subfolder, exclude_path) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn picker_start_naming(&mut self) {
        self.picker_mode = PickerMode::Naming;
        self.picker_name_input = String::new();
        self.picker_name_cursor = 0;
    }

    pub fn picker_cancel_naming(&mut self) {
        self.picker_mode = PickerMode::Selecting;
    }

    pub fn picker_start_renaming(&mut self) {
        if self.picker_entries.is_empty() {
            return;
        }

        let selected = self.picker_selected.min(self.picker_entries.len() - 1);
        let entry = self.picker_entries[selected].clone();

        if let PickerEntry::Request { name, path } = entry {
            self.picker_mode = PickerMode::Renaming;
            self.picker_name_input = name.clone();
            self.picker_name_cursor = name.len();
            self.picker_rename_path = path;
        }
    }

    pub fn picker_rename_request(&mut self, new_name: String) {
        let new_name = new_name.trim().to_string();
        if new_name.is_empty() {
            return;
        }

        let Some(requests_dir) = Self::requests_dir() else {
            return;
        };

        let old_file = requests_dir.join(format!("{}.json", self.picker_rename_path));

        // Build new path
        let new_path = if self.picker_current_folder.is_empty() {
            new_name.clone()
        } else {
            format!("{}/{}", self.picker_current_folder, new_name)
        };

        let new_file = requests_dir.join(format!("{}.json", new_path));

        // Create parent dirs if needed
        if let Some(parent) = new_file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // Rename the file
        if std::fs::rename(&old_file, &new_file).is_err() {
            return;
        }

        // Clean up empty old parent dir
        if let Some(parent) = old_file.parent() {
            if parent != requests_dir {
                if let Ok(mut entries) = std::fs::read_dir(parent) {
                    if entries.next().is_none() {
                        let _ = std::fs::remove_dir(parent);
                    }
                }
            }
        }

        // If renamed request is the active one, update identity
        if self.picker_rename_path == self.request_path {
            self.request_path = new_path;
            self.request_name = self
                .request_path
                .rsplit('/')
                .next()
                .unwrap_or(&self.request_path)
                .to_string();
            self.save_active_path();
        }

        self.picker_entries = Self::list_folder(&self.picker_current_folder);
        if self.picker_selected >= self.picker_entries.len() && !self.picker_entries.is_empty() {
            self.picker_selected = self.picker_entries.len() - 1;
        }
        self.picker_mode = PickerMode::Selecting;
    }

    // --- HTTP Request ---

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

    // --- Text editing helpers ---

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
