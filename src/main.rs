mod api;
mod app;
mod ui;

use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use reqwest::Client;

use std::{error::Error, io};

use app::{App, CurrentScreen};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let client = Client::new();
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }

                    KeyCode::Char('l') => {
                        app.current_screen = CurrentScreen::Login;
                    }
                    _ => {}
                },

                CurrentScreen::Login if key.kind == event::KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Backspace => {
                        app.instance_url.pop();
                    }

                    KeyCode::Esc => {
                        app.instance_url = String::from("");
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Char(value) => app.instance_url.push(value),

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
