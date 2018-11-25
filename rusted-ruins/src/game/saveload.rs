
use std::fs;
use std::path::{Path, PathBuf};
use common::basic::{SAVE_DIR_NAME, SAVE_EXTENSION};
use common::gamedata::GameData;
use config::USER_DIR;
use game::{Game, InfoGetter};

impl Game {
    pub fn save_file(&self) {
        let save_dir = get_save_dir();

        if !save_dir.exists() {
            match fs::create_dir_all(&save_dir) {
                Ok(()) => (),
                Err(e) => {
                    warn!("Failed to create save directory : {}", e);
                    return;
                }
            }
        }
        
        let save_file_name = format!("{}.{}", self.gd.player_name(), SAVE_EXTENSION);

        let path = save_dir.join(save_file_name);
        
        match self.gd.save_file(&path) {
            Ok(_) => info!("Saved to {:?}", path.to_string_lossy()),
            Err(e) => warn!("Faild to saving to {:?}: {}", path.to_string_lossy(), e),
        }
    }
}

pub fn save_file_list() -> Result<Vec<PathBuf>, ::std::io::Error> {
    let mut list = Vec::new();
    
    for entry in fs::read_dir(get_save_dir())? {
        let file = entry?;

        if !file.file_type()?.is_file() {
            continue;
        }

        let path = file.path();

        // let extension = path.extension(); // TODO: May use this line with NLL
        let path_clone = path.clone();
        let extension = path_clone.extension();
        
        if extension.is_some() && extension.unwrap() == SAVE_EXTENSION {
            list.push(path);
        }
    }

    Ok(list)
}

fn get_save_dir() -> PathBuf {
    USER_DIR.clone().join(SAVE_DIR_NAME)
}

