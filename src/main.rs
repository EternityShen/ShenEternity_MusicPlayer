use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use musicplayer::{data::app, ui};
use ratatui::{Terminal, prelude::CrosstermBackend};
fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::default();
    loop {
        terminal.draw(|frame| {
            ui::draw(frame, &mut app);
        })?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            if key.code == KeyCode::Char('j') {
                app.next_tab();
            }
            if KeyCode::Char('q') == key.code {
                app.quit();
            }
            if key.code == KeyCode::Char('k') {
                app.select_prev();
            }
            if key.code == KeyCode::Char('l') {
                app.select_next();
            }
            if key.code == KeyCode::Char('p') {
                app.play_or_pause();
            }
        }

        app.auto_next();

        if app.should_quit {
            break;
        }
    }
    let mut stdout = std::io::stdout();
    disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
}
