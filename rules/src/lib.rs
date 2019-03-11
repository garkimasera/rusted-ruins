#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate rusted_ruins_array2d as array2d;
extern crate rusted_ruins_common as common;

pub mod chara;
pub mod charagen;
pub mod creation;
pub mod dungeon_gen;
pub mod exp;
pub mod floor_gen;
pub mod newgame;
pub mod params;
pub mod quest;
pub mod town;

use lazy_static::lazy_static;
use serde::de::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Contain game rules
pub struct Rules {
    pub chara: chara::Chara,
    pub chara_gen: charagen::CharaGen,
    pub creation: creation::Creation,
    pub dungeon_gen: dungeon_gen::DungeonGen,
    pub exp: exp::Exp,
    pub floor_gen: floor_gen::FloorGen,
    pub newgame: newgame::NewGame,
    pub params: params::Params,
    pub quest: quest::Quest,
    pub town: town::Town,
}

impl Rules {
    fn load_from_dir(rules_dir: &Path) -> Rules {
        let mut creation: creation::Creation = read_from_json(&rules_dir.join("creation.json"));
        creation.sort();

        Rules {
            chara: read_from_json(&rules_dir.join("chara.json")),
            chara_gen: read_from_json(&rules_dir.join("charagen.json")),
            creation,
            dungeon_gen: read_from_json(&rules_dir.join("dungeon_gen.json")),
            exp: read_from_json(&rules_dir.join("exp.json")),
            floor_gen: read_from_json(&rules_dir.join("floor_gen.json")),
            newgame: read_from_json(&rules_dir.join("newgame.json")),
            params: read_from_json(&rules_dir.join("params.json")),
            quest: read_from_json(&rules_dir.join("quest.json")),
            town: read_from_json(&rules_dir.join("town.json")),
        }
    }
}

fn read_from_json<T: for<'de> Deserialize<'de>>(file_path: &Path) -> T {
    info!("Rule file loading: \"{}\"", file_path.to_string_lossy());
    let file = match fs::File::open(file_path) {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            exit_err()
        }
    };
    match serde_json::from_reader(file) {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            exit_err()
        }
    }
}

fn exit_err() -> ! {
    std::process::exit(1)
}

lazy_static! {
    static ref RULES_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    /// Global state rules holder
    pub static ref RULES: Rules = Rules::load_from_dir(&RULES_DIR.lock().unwrap().as_ref().unwrap());
}

/// Initialize Rules
pub fn init<P: AsRef<Path>>(app_dirs: P) {
    let rules_dir = app_dirs.as_ref().join("rules");

    *RULES_DIR.lock().unwrap() = Some(rules_dir);

    lazy_static::initialize(&RULES);
}
