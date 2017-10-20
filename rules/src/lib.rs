
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate rusted_ruins_common as common;

pub mod mapgen;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::fs;
use serde::de::Deserialize;

/// Contain game rules
pub struct Rules {
    pub mapgen: mapgen::MapGen,
}

impl Rules {
    fn load_from_dir(rules_dir: &Path) -> Rules {
        Rules {
            mapgen: read_from_json(&rules_dir.join("mapgen.json"))
        }
    }
}

fn read_from_json<T: for<'de> Deserialize<'de>>(file_path: &Path) -> T {
    info!("Rule file loading: \"{}\"", file_path.to_string_lossy());
    let file = match fs::File::open(file_path) {
        Ok(o) => o,
        Err(e) => { error!("{}", e); exit_err() },
    };
    match serde_json::from_reader(file) {
        Ok(o) => o,
        Err(e) => { error!("{}", e); exit_err() },
    }
}

fn exit_err() -> ! {
    ::std::process::exit(1)
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
