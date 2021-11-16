use sdl2::mixer::{self, Channel, Chunk};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct WavTable {
    channel: Channel,
    chunks: HashMap<String, Chunk>,
}

impl WavTable {
    pub fn new<P: AsRef<Path>>(data_dirs: &[P], volume: i32) -> WavTable {
        // Load chunks from datadir/sound
        let mut chunks = HashMap::new();

        for data_dir in data_dirs {
            let sound_dir = data_dir.as_ref().join("sound");
            let sound_dir = warn_continue!(fs::read_dir(sound_dir));

            for entry in sound_dir {
                let entry = warn_continue!(entry);

                // If this file isn't wav
                let path = entry.path();
                if path.extension().is_none() || path.extension().unwrap() != "wav" {
                    continue;
                }

                let chunk = warn_continue!(Chunk::from_file(&path));
                let filename = warn_continue!(path.file_stem().ok_or("Invalid file name"))
                    .to_string_lossy()
                    .into_owned();

                chunks.insert(filename, chunk);
            }
        }

        let channel = mixer::Channel(0);
        channel.set_volume(volume);

        WavTable { channel, chunks }
    }

    pub fn play(&self, name: &str) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get(name) {
            self.channel.halt();
            self.channel.play(chunk, 0)?;
            Ok(())
        } else {
            Err(format!("Unknown sound effect \"{}\"", name))
        }
    }
}
