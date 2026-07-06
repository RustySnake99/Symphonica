use rodio::{DeviceSinkBuilder, Player, Decoder};
use std::{fs::File, sync::{Arc, Mutex}};

pub struct AudioPlayer {
    _sink: rodio::MixerDeviceSink,   // must be kept alive or audio stops
    player: Arc<Mutex<Player>>,
}

impl AudioPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let sink = DeviceSinkBuilder::open_default_sink()?;
        let mixer = sink.mixer();
        let player = Player::connect_new(&mixer);
        Ok(Self {
            _sink: sink,
            player: Arc::new(Mutex::new(player)),
        })
    }

    pub fn play(&self, path: &str) -> anyhow::Result<()> {
        let player = self.player.lock().unwrap();
        player.stop();  // clear current queue
        let file = File::open(path)?;
        let source = Decoder::try_from(file)?;
        player.append(source);
        player.play();
        Ok(())
    }

    pub fn toggle_pause(&self) {
        let player = self.player.lock().unwrap();
        if player.is_paused() { player.play(); } else { player.pause(); }
    }

    pub fn is_paused(&self) -> bool {
        self.player.lock().unwrap().is_paused()
    }

    pub fn set_volume(&self, v: f32) {
        self.player.lock().unwrap().set_volume(v.clamp(0.0, 2.0));
    }

    pub fn volume(&self) -> f32 {
        self.player.lock().unwrap().volume()
    }

    pub fn is_finished(&self) -> bool {
        self.player.lock().unwrap().empty()
    }

    pub fn stop(&self) {
        self.player.lock().unwrap().stop();
    }
}