use crate::{audio::AudioPlayer, library::Track};

pub struct App {
    pub tracks: Vec<Track>,
    pub selected: usize,
    pub player: AudioPlayer,
    pub current_track: Option<usize>,
    pub volume: f32,
    pub search: String,
    pub search_mode: bool,
}

impl App {
    pub fn new(tracks: Vec<Track>, player: AudioPlayer) -> Self {
        Self {
            tracks, player,
            selected: 0,
            current_track: None,
            volume: 1.0,
            search: String::new(),
            search_mode: false,
        }
    }
    pub fn play_selected(&mut self) {
        let idx = self.selected;
        
        if let Some(track) = self.tracks.get(idx) {
            let _ = self.player.play(&track.path.clone());
            self.current_track = Some(idx);
        }
    }
    pub fn next(&mut self) {
        if !self.tracks.is_empty() {self.selected = (self.selected + 1) % self.tracks.len();}
    }
    pub fn previous(&mut self) {
        if !self.tracks.is_empty() {self.selected = self.selected.saturating_sub(1);}
    }
    pub fn volume_up(&mut self) {
        self.volume = (self.volume + 0.1).min(2.0);
        self.player.set_volume(self.volume);
    }
    pub fn volume_down(&mut self) {
        self.volume = (self.volume - 0.1).max(0.0);
        self.player.set_volume(self.volume);
    }
    pub fn filtered_tracks(&self) -> Vec<(usize, &Track)> {
        self.tracks.iter().enumerate().filter(|(_, t)| {
            let q = self.search.to_lowercase();
            q.is_empty()
                || t.title.to_lowercase().contains(&q)
                || t.artist.to_lowercase().contains(&q)
        }).collect()
    }
}