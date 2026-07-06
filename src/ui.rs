use crate::app::App;
use ratatui::{layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Min(5),
        Constraint::Length(5),
        Constraint::Length(1),
    ]).split(f.area());

    draw_library(f, app, chunks[0]);
    draw_now_playing(f, app, chunks[1]);
    draw_help(f, chunks[2]);
}

fn draw_library(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let filtered = app.filtered_tracks();
    let items: Vec<ListItem> = filtered.iter().map(|(i, t)| {
        let is_playing = app.current_track == Some(*i);
        let prefix = if is_playing { "▶ " } else { "  " };
        let style = if is_playing {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(Line::from(vec![
            Span::styled(format!("{}{}", prefix, t.title), style),
            Span::styled(format!("  —  {}", t.artist), Style::default().fg(Color::DarkGray)),
        ]))
    }).collect();

    let title = if app.search_mode {
        format!("🔍 Search: {}_", app.search)
    } else {
        format!("🎵 Library  ({} tracks)", app.tracks.len())
    };

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_now_playing(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);
    let (title, artist, album) = app.current_track.and_then(|i| app.tracks.get(i)).map(|t| (t.title.as_str(), t.artist.as_str(), t.album.as_str())).unwrap_or(("No Track Playing!", "", ""));

    let status = if app.player.is_paused() {"⏸ Paused"} else {"▶ Playing"};
    let info = Paragraph::new(vec![
        Line::from(Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("{} · {}", artist, album), Style::default().fg(Color::Gray))),
        Line::from(Span::styled(status, Style::default().fg(Color::Yellow))),
    ]).block(Block::default().borders(Borders::ALL).title("Now Playing"));

    f.render_widget(info, chunks[0]);

    let volume_pct = (app.player.volume() * 50.0) as u16;
    let gauge = Gauge::default().block(Block::default().borders(Borders::ALL).title("🔊 Volume")).gauge_style(Style::default().fg(Color::Green)).percent(volume_pct.min(100));

    f.render_widget(gauge, chunks[1]);
}

fn draw_help(f: &mut Frame, area: ratatui::layout::Rect) {
    let help = Paragraph::new(Line::from(vec![
        hint("Enter", "Play"), hint("Space", "Pause"), hint("↑↓", "Navigate"), hint("+/-", "Volume"), hint("/", "Search"), hint("q", "Quit"),
    ]));
    f.render_widget(help, area);
}

fn hint<'a>(key: &'a str, desc: &'a str) -> Span<'a> {
    Span::raw(format!("  [{}] {}  ", key, desc))
}