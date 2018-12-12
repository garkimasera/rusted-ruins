
use std::fs;
use std::path::PathBuf;
use common::basic::{SAVE_EXTENSION, SAVE_DIR_NAME};
use common::gamedata::GameData;
use crate::config::USER_DIR;
use crate::game::{Game, InfoGetter};

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

        let path = self.gd.save_dir(save_dir);
        
        match self.gd.save(&path) {
            Ok(_) => info!("Saved to {:?}", path.to_string_lossy()),
            Err(e) => warn!("Faild to saving to {:?}: {}", path.to_string_lossy(), e),
        }
    }
}

pub fn save_file_list() -> Result<Vec<PathBuf>, ::std::io::Error> {
    let mut list = Vec::new();
    
    for entry in fs::read_dir(get_save_dir())? {
        let file = entry?;

        if !file.file_type()?.is_dir() {
            continue;
        }

        let path = file.path();

        let extension = path.extension();
        
        if extension.is_some() && extension.unwrap() == SAVE_EXTENSION {
            list.push(path);
        }
    }

    Ok(list)
}

/// Generate random id for FileBox
pub fn gen_box_id(_gd: &GameData) -> u64 {
    use rng::*;
    thread_rng().gen::<u64>()
}

fn get_save_dir() -> PathBuf {
    USER_DIR.clone().join(SAVE_DIR_NAME)
}

/// Get each save directory path "save_dir/save_name"
pub fn get_each_save_dir(gd: &GameData) -> PathBuf {
    get_save_dir().join(format!("{}.{}", gd.meta.save_name(), SAVE_EXTENSION))
}

