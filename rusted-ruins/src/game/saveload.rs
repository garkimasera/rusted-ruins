
use std::fs;
use std::path::Path;
use common::basic::{SAVE_DIR_NAME, SAVE_EXTENSION};
use common::gamedata::GameData;
use config::USER_DIR;
use game::{Game, InfoGetter};

impl Game {
    pub fn save_file(&self) {
        let save_dir = USER_DIR.clone().join(SAVE_DIR_NAME);

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

    pub fn load_file<P: AsRef<Path>>(path: P) -> Game {
        let gd = GameData::load_file(path).unwrap();

        let mut game = Game::empty();
        game.gd = gd;
        game
    }
}

