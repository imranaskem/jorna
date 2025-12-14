use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Entity, SharedString,
    Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
    Disableable, Root,
};

struct HttpClient {
    url_input: Entity<InputState>,
    response: SharedString,
    loading: bool,
}

impl HttpClient {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("https://pokeapi.co/api/v2/pokemon/snorlax")
                .placeholder("Enter URL...")
        });

        Self {
            url_input,
            response: "Response will appear here...".into(),
            loading: false,
        }
    }

    fn send_request(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_input.read(cx).text().to_string();
        self.loading = true;
        self.response = "Loading...".into();
        cx.notify();

        // Simple synchronous request - much faster than recreating editor components
        let response_text = match reqwest::blocking::get(&url) {
            Ok(response) => {
                let status = response.status();
                let headers = format!("Status: {}\n\n", status);
                match response.text() {
                    Ok(body) => format!("{}{}", headers, body),
                    Err(e) => format!("Error reading response: {}", e),
                }
            }
            Err(e) => format!("Request failed: {}", e),
        };

        self.response = response_text.into();
        self.loading = false;
        cx.notify();
    }

    fn on_send_click(&mut self, _event: &gpui::ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !self.loading {
            self.send_request(window, cx);
        }
    }
}

impl Render for HttpClient {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .p_4()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_xl().text_color(rgb(0xffffff)).child("HTTP Client"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .child(Input::new(&self.url_input)),
                            )
                            .child(
                                Button::new("send-button")
                                    .primary()
                                    .label(if self.loading { "Sending..." } else { "Send" })
                                    .disabled(self.loading)
                                    .on_click(cx.listener(Self::on_send_click)),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .p_3()
                    .bg(rgb(0x2d2d2d))
                    .border_1()
                    .border_color(rgb(0x3d3d3d))
                    .rounded_md()
                    .text_color(rgb(0xcccccc))
                    .font_family(".SystemUIFont")
                    .text_sm()
                    .child(self.response.clone()),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(|cx: &mut App| {
        // Initialize gpui-component
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| HttpClient::new(window, cx));
                // Wrap in Root component - this is required for gpui-component
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();
    });
}
