# 🎵 Symphonica

A fast, lightweight terminal music player written in Rust.

Symphonica scans your local music library, extracts metadata, and provides an elegant terminal interface for browsing and playing music without leaving the command line.

---

## Features

- 🎵 Browse your music library
- 🔍 Search by title or artist
- ▶ Play and pause tracks
- 🔊 Adjustable volume control
- 📁 Recursive directory scanning
- 🏷 Automatic metadata extraction
- 🎨 Modern terminal UI powered by Ratatui
- 🎧 Supports multiple audio formats:
  - MP3
  - FLAC
  - WAV
  - OGG
  - AAC
  - M4A

---

## Built With

- Rust
- Ratatui
- Crossterm
- Rodio
- Lofty
- WalkDir
- Tokio
- Anyhow

---

## Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/Symphonica.git
cd Symphonica
```

Build the project:

```bash
cargo build --release
```

Run:

```bash
cargo run
```

---

## Music Library

By default Symphonica scans a folder named:

```
Tracks/
```

Place your music files inside:

```
Symphonica/
│
├── Tracks/
│   ├── song1.mp3
│   ├── song2.flac
│   └── ...
└── src/
```

---

## Controls

| Key | Action |
|------|--------|
| ↑ / k | Previous track |
| ↓ / j | Next track |
| Enter | Play selected track |
| Space | Pause / Resume |
| + | Increase volume |
| - | Decrease volume |
| / | Search library |
| Esc | Exit search |
| q | Quit |

---

## Project Structure

```
src/
├── app.rs       # Application state
├── audio.rs     # Audio playback
├── library.rs   # Music discovery & metadata
├── ui.rs        # Terminal interface
└── main.rs      # Event loop
```

---

## Supported Formats

- MP3
- FLAC
- WAV
- OGG
- AAC
- M4A

---

## Future Plans

- Playlist support
- Shuffle mode
- Repeat mode
- Progress bar
- Theme customization
- Configuration file
- Album art
- Queue management
- Keyboard remapping

---

## License

MIT License

---

## Author

Built with Rust ❤️
