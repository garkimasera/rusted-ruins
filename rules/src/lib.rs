
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate lazy_static;

use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Contain game rules
pub struct Rules;

impl Rules {
    fn load_from_dir(rules_dir: &Path) -> Rules {
        Rules
    }
}




// Global state rules holder

lazy_static! {
    static ref RULES_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref RULES: Rules = Rules::load_from_dir(&RULES_DIR.lock().unwrap().as_ref().unwrap());
}

/// Initialize Rules
pub fn init<P: AsRef<Path>>(app_dirs: P) {
    let rules_dir = app_dirs.as_ref().join("rules");

    *RULES_DIR.lock().unwrap() = Some(rules_dir);

    lazy_static::initialize(&RULES);
}
