use crossterm::{
    event::{self as crossterm_event, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};

mod app;
mod ui;
mod event;

use app::App;
use ui::ui;
use event::handle_key_event;

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if app.should_quit {
            break;
        }

        // Poll for events with timeout
        if crossterm_event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm_event::read()? {
                // Only handle KeyPress events (ignore KeyRelease)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                handle_key_event(&mut app, key);
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Setup panic hook for guaranteed cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Initialize app state
    let app = App::new();

    // Run app with proper error handling
    let result = run_app(&mut terminal, app);

    // Cleanup terminal (even on error)
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    // Return error after cleanup
    result?;

    Ok(())
}
