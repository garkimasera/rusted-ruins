use sdl2::mixer::Music;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct MusicTable {
    music: HashMap<String, Music<'static>>,
    current_music: RefCell<Option<String>>,
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
                if path.extension().is_none() {
                    continue;
                }
                let extension = path.extension().unwrap();

                if extension == "opus" {
                    let m = warn_continue!(Music::from_file(&path));
                    let filename = warn_continue!(path.file_stem().ok_or("Invalid file name"))
                        .to_string_lossy()
                        .into_owned();

                    music.insert(filename, m);
                } else if extension == "dat" {
                    warn_continue!(crate::datwalker::datwalker(
                        &path,
                        "opus",
                        |filename, data| {
                            let m = match Music::from_static_bytes(data) {
                                Ok(m) => m,
                                Err(e) => {
                                    warn!("{}", e);
                                    return;
                                }
                            };
                            music.insert(filename, m);
                        }
                    ));
                }
            }
        }

        MusicTable {
            music,
            current_music: RefCell::new(None),
        }
    }

    pub fn play(&self, name: &str) -> Result<(), String> {
        let mut current_music = self.current_music.borrow_mut();
        if current_music.is_some() && current_music.as_ref().unwrap() == name {
            return Ok(());
        }

        if let Some(m) = self.music.get(name) {
            m.play(-1)?; // Pass -1 for infinite loop
            *current_music = Some(name.to_owned());
            Ok(())
        } else {
            Err(format!("Unknown music \"{name}\""))
        }
    }
}
