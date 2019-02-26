mod args;
pub mod font;
pub mod input;
pub mod visual;

use crate::util::read_file_as_string;
use common::basic;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use toml;

macro_rules! load_config_file {
    ($path:expr) => {{
        let path = cfg_path($path);
        info!("Loading config file : \"{}\"", path.to_string_lossy());
        let s = match read_file_as_string(&path) {
            Ok(s) => s,
            Err(e) => {
                error!(
                    "Cannot load config file \"{}\"\n{}",
                    path.to_string_lossy(),
                    e
                );
                exit(1);
            }
        };

        match toml::de::from_str(&s) {
            Ok(config) => config,
            Err(e) => {
                error!(
                    "Cannot load config file \"{}\"\n{}",
                    path.to_string_lossy(),
                    e
                );
                exit(1);
            }
        }
    }};
}

/// Initialize lazy static
pub fn init() {
    use lazy_static::initialize;
    initialize(&APP_DIR);
    initialize(&USER_DIR);
    initialize(&CONFIG);
    initialize(&SCREEN_CFG);
    initialize(&UI_CFG);
    initialize(&INPUT_CFG);
    initialize(&PAK_DIRS);
}

lazy_static! {
    pub static ref APP_DIR: PathBuf = get_app_dir().expect("Cannot get data directory path");
    pub static ref USER_DIR: PathBuf = get_user_dir();
    pub static ref ADDON_DIR: Option<PathBuf> = get_addon_dir();
    pub static ref CONFIG: Config = {
        let config: Config = load_config_file!("config.toml");
        args::modify_config_by_args(config)
    };
    pub static ref SCREEN_CFG: visual::ScreenConfig = load_config_file!("screen/800x600.toml");
    pub static ref UI_CFG: visual::UIConfig = load_config_file!("ui.toml");
    pub static ref INPUT_CFG: input::InputConfig = load_config_file!("input.toml");
    pub static ref FONT_CFG: font::FontConfig = load_config_file!("font.toml");
    pub static ref PAK_DIRS: Vec<PathBuf> = {
        let mut v = Vec::new();
        v.push(abs_path("paks"));
        v
    };
}

/// Get application directory
fn get_app_dir() -> Option<PathBuf> {
    if let Some(e) = env::var_os("RUSTED_RUINS_APP_DIR") {
        return Some(PathBuf::from(e));
    }

    if let Ok(mut exe_file) = env::current_exe() {
        exe_file.pop();
        exe_file.push("data");
        return Some(exe_file);
    }

    if let Ok(mut cdir) = env::current_dir() {
        cdir.push("data");
        return Some(cdir);
    }
    None
}

fn get_user_dir() -> PathBuf {
    let mut path = dirs::data_dir().expect("Failed to get user data diractory");
    path.push(basic::APP_DIR_NAME);
    path
}

/// Get addon directory
fn get_addon_dir() -> Option<PathBuf> {
    if let Some(e) = env::var_os("RUSTED_RUINS_ADDON_DIR") {
        return Some(PathBuf::from(e));
    }
    None
}

/// Get application and each addon's directories
/// They will be the root path for searching pak or text, and other data files.
pub fn get_data_dirs() -> Vec<PathBuf> {
    let mut v = Vec::new();
    v.push(APP_DIR.clone());

    if ADDON_DIR.is_some() {
        v.push(ADDON_DIR.clone().unwrap());
    }

    v
}

/// Create absolute path from relative path which root is application directory
pub fn abs_path(s: &str) -> PathBuf {
    let mut path = APP_DIR.clone();
    path.push(s);
    path
}

/// Create absolute path from config directory
pub fn cfg_path(s: &str) -> PathBuf {
    let mut path = APP_DIR.clone();
    path.push(basic::CFG_FILES_DIR);
    path.push(s);
    path
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CfgRect {
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub w: u32,
    #[serde(default)]
    pub h: u32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CfgPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CfgColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub lang: String,
    pub second_lang: String,
    pub screen_config: String,
    pub hardware_acceleration: bool,
    #[serde(default)]
    pub double_scale_mode: bool,
    #[serde(default)]
    pub fix_rand: bool,
}
