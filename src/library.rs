use lofty::{tag::Accessor, file::TaggedFileExt, read_from_path};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Track {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
}

const SUPPORTED: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a", "aac"];

pub fn scan_directory(dir: &str) -> Vec<Track> {
    WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).filter(|e| {
        e.path().extension().and_then(|x| x.to_str()).map(|ext| SUPPORTED.contains(&ext.to_lowercase().as_str())).unwrap_or(false)
    }).map(|e| parse_track(e.path().to_str().unwrap_or(""))).collect()
}

fn parse_track(path: &str) -> Track {
    let tagged = read_from_path(path).ok();

    let (title, artist, album) = tagged.as_ref().and_then(|f| f.primary_tag()).map(|tag| {
        (
            tag.title().map(|s| s.to_string()).unwrap_or_else(|| filename(path)),
            tag.artist().map(|s| s.to_string()).unwrap_or_else(|| "Unknown".into()),
            tag.album().map(|s| s.to_string()).unwrap_or_else(|| "Unknown".into()),
        )
    }).unwrap_or_else(|| (filename(path), "Unknown".into(), "Unknown".into()));

    Track {path: path.to_string(), title, artist, album}
}

fn filename(path: &str) -> String {
    std::path::Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string()
}