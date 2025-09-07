// Music library to enumerate and play .ogg music files using the rodio audio backend.
// Public API kept minimal and dependency-free for consumers besides rodio.

use std::path::{Path, PathBuf};
use std::fs;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct MusicFile {
    pub path: PathBuf,
}

impl MusicFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self, String> {
        let p = path.into();
        if !p.exists() {
            return Err(format!("Music file not found: {}", p.display()));
        }
        if p.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("ogg")).unwrap_or(false) {
            Ok(MusicFile { path: p })
        } else {
            Err(format!("Unsupported file extension for {}. Only .ogg is supported.", p.display()))
        }
    }
}

#[derive(Debug, Default)]
pub struct MusicLibrary {
    tracks: Vec<MusicFile>,
}

impl MusicLibrary {
    pub fn new() -> Self { Self { tracks: Vec::new() } }

    pub fn from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, String> {
        let mut lib = MusicLibrary::new();
        let entries = fs::read_dir(&dir).map_err(|e| format!("Failed to read dir {}: {}", dir.as_ref().display(), e))?;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        if ext.eq_ignore_ascii_case("ogg") {
                            if let Ok(file) = MusicFile::new(path.clone()) {
                                lib.tracks.push(file);
                            }
                        }
                    }
                }
            }
        }
        Ok(lib)
    }

    pub fn add<P: Into<PathBuf>>(&mut self, path: P) -> Result<(), String> {
        let file = MusicFile::new(path)?;
        self.tracks.push(file);
        Ok(())
    }

    pub fn list(&self) -> &[MusicFile] { &self.tracks }

    pub fn is_empty(&self) -> bool { self.tracks.is_empty() }
}

// Play the provided .ogg file. This call blocks until playback finishes.
pub fn play(file: &MusicFile) -> Result<(), String> {
    // Double-check file exists and is .ogg
    if !file.path.exists() {
        return Err(format!("File does not exist: {}", file.path.display()));
    }
    if !file.path.is_file() {
        return Err(format!("Not a file: {}", file.path.display()));
    }
    if !file.path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("ogg")).unwrap_or(false) {
        return Err(format!("Not an .ogg file: {}", file.path.display()));
    }

    // Use rodio to play the OGG file
    let device = rodio::default_output_device().ok_or_else(|| "No default audio output device available".to_string())?;
    let sink = rodio::Sink::try_new(&device).map_err(|e| format!("Failed to create audio sink: {}", e))?;

    let file_handle = std::fs::File::open(&file.path).map_err(|e| format!("Failed to open {}: {}", file.path.display(), e))?;
    let source = rodio::Decoder::new(BufReader::new(file_handle)).map_err(|e| format!("Failed to decode {}: {}", file.path.display(), e))?;

    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

// Non-blocking variant: starts playback on a background thread and returns immediately.
// Returns a handle that can be used to stop the playback early if needed.
pub struct PlayHandle {
    sink: rodio::Sink,
    _guard: std::thread::JoinHandle<()>,
}

impl PlayHandle {
    pub fn stop(self) {
        self.sink.stop();
        // dropping will end the thread once sink is stopped
    }
}

pub fn play_nonblocking(file: &MusicFile) -> Result<PlayHandle, String> {
    if !file.path.exists() {
        return Err(format!("File does not exist: {}", file.path.display()));
    }
    if !file.path.is_file() {
        return Err(format!("Not a file: {}", file.path.display()));
    }
    if !file.path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("ogg")).unwrap_or(false) {
        return Err(format!("Not an .ogg file: {}", file.path.display()));
    }

    let device = rodio::default_output_device().ok_or_else(|| "No default audio output device available".to_string())?;
    let sink = rodio::Sink::try_new(&device).map_err(|e| format!("Failed to create audio sink: {}", e))?;

    let path = file.path.clone();
    let sink_clone = sink.clone();
    let handle = std::thread::spawn(move || {
        if let Ok(f) = fs::File::open(&path) {
            if let Ok(decoder) = rodio::Decoder::new(BufReader::new(f)) {
                sink_clone.append(decoder);
                sink_clone.sleep_until_end();
            }
        }
    });

    Ok(PlayHandle { sink, _guard: handle })
}
