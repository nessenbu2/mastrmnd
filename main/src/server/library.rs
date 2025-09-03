use std::fs;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    artist: String,
    album: String,
    name: String,
}

impl Song {
    pub fn new(artist: String, album: String, name: String) -> Song {
        Song {
            artist,
            album,
            name
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    songs: Vec<Song>,
    name: String
}

impl Album {
    pub fn new(name: String) -> Album {
        Album {
            name,
            songs: Vec::new()
        }
    }

    pub fn print_songs(&self) {
        for song in self.songs.iter() {
            println!("{:?}", song.name);
        }
    }

    pub fn get_song(&self, song: &String) -> Option<&Song> {
        self.songs.iter().find(|&s| s.name == *song)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    albums: Vec<Album>,
    name: String
}

impl Artist {
    pub fn new(name: String) -> Artist {
        Artist {
            name: name,
            albums: Vec::new()
        }
    }

    pub fn get_album(&self, album: &String) -> Option<&Album> {
        self.albums.iter().find(|&a| a.name == *album)
    }

    pub fn print_albums(&self) {
        for album in self.albums.iter() {
            println!("{:?}", album.name);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Library{
    artists: Arc<Mutex<HashMap<String, Artist>>>
}

impl Library {
    pub fn print_artists(&self) {
        for (_, artist) in self.artists.lock().unwrap().iter() {
            println!("{:?}", artist.name);
        }
    }

    pub fn get_artist(&self, artist: String) -> Option<Artist>{
        self.artists.lock().unwrap().get(&artist).cloned()
    }

    pub fn get_state(&self) -> Vec<Artist> {
        self.artists.lock().unwrap().values().map(|a| a.clone()).collect()
    }

    pub fn init(base_dir:String) -> Library {
        let mut lib = Library::default();

        // this breaks if files aren't set up correctly
        let artists = fs::read_dir(base_dir).unwrap();

        for artist in artists {
            let artist = artist.unwrap();
            let mut _artist = Artist::new(artist.file_name().into_string().unwrap());
            let albums = fs::read_dir(artist.path()).unwrap();
            for album in albums {
                let album_path = album.unwrap();
                let mut _album = Album::new(album_path.file_name().into_string().unwrap());
                let songs = fs::read_dir(album_path.path()).unwrap();
                for song in songs {
                    let song_name = song.unwrap().file_name().into_string().unwrap();
                    _album.songs.push(Song::new(
                            _artist.name.clone(),
                            _album.name.clone(),
                            song_name.clone()));
                }
                _artist.albums.push(_album);
            }
            lib.artists.lock().unwrap().insert(_artist.name.clone(), _artist);
        }

        // dump files for debugging
        for (_, artist) in lib.artists.lock().unwrap().iter() {
            println!("{:?}", artist.name);
            for album in artist.albums.iter() {
                println!("   {:?}", album.name);
                for song in album.songs.iter() {
                    println!("        {:?}", song.name);
                }
            }
        }
        lib
    }
}
