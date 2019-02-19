use sdl2::mixer::Music;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct MusicTable {
    music: HashMap<String, Music<'static>>,
}

impl MusicTable {
    pub fn new<P: AsRef<Path>>(data_dirs: &[P]) -> MusicTable {
        // Load music from datadir/music
        let mut music = HashMap::new();

        for data_dir in data_dirs {
            let music_dir = data_dir.as_ref().join("music");
            let music_dir = warn_continue!(fs::read_dir(music_dir));

            for entry in music_dir {
                let entry = warn_continue!(entry);

                // If this file isn't ogg
                let path = entry.path();
                if path.extension().is_none() || path.extension().unwrap() != "ogg" {
                    continue;
                }

                let m = warn_continue!(Music::from_file(&path));
                let filename = warn_continue!(path.file_stem().ok_or("Invalid file name"))
                    .to_string_lossy()
                    .into_owned();

                music.insert(filename, m);
            }
        }

        MusicTable { music }
    }

    pub fn play(&self, name: &str) -> Result<(), String> {
        if let Some(m) = self.music.get(name) {
            m.play(-1)?; // Pass -1 for infinite loop
            Ok(())
        } else {
            Err(format!("Unknown music \"{}\"", name))
        }
    }
}
