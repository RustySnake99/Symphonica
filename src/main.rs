mod library;
mod audio;
mod app;
mod ui;

use app::App;
use audio::AudioPlayer;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> anyhow::Result<()> {
    let dir = String::from("Tracks");
    let tracks = library::scan_directory(&dir);
    let player = AudioPlayer::new()?;
    let mut app = App::new(tracks, player);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);  // <-- extract loop

    // Always restore, even if run() returned Err
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if app.current_track.is_some() && app.player.is_finished() {
            app.next();
            app.play_selected();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                if app.search_mode {
                    match key.code {
                        KeyCode::Esc       => { app.search_mode = false; app.search.clear(); }
                        KeyCode::Enter     => { app.search_mode = false; }
                        KeyCode::Backspace => { app.search.pop(); }
                        KeyCode::Char(c)   => { app.search.push(c); }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => {app.player.stop(); return Ok(());},
                        KeyCode::Enter     => app.play_selected(),
                        KeyCode::Char(' ') => app.player.toggle_pause(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Char('+') | KeyCode::Char('=') => app.volume_up(),
                        KeyCode::Char('-') => app.volume_down(),
                        KeyCode::Char('/') => app.search_mode = true,
                        _ => {}
                    }
                }
            }
        }
    }
}