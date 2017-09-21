
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use sdl2::mixer::{Channel, Chunk};

pub struct WavTable {
    chunks: HashMap<String, Chunk>,
}

impl WavTable {
    pub fn new<P: AsRef<Path>>(app_dirs: &[P]) -> WavTable {
        // Load chunks from appdir/sound
        let mut chunks = HashMap::new();
        
        for app_dir in app_dirs {
            let sound_dir = app_dir.as_ref().join("sound");
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
                    .to_string_lossy().into_owned();

                chunks.insert(filename, chunk);
            }
        }
        
        WavTable {
            chunks,
        }
    }

    pub fn play(&self, s: &str) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get(s) {
            Channel::all().play(&chunk, 0)?;
            Ok(())
        }else{
            Err(format!("Unknown sound effect \"{}\"", s))
        }
    }
}

