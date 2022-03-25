mod args;
pub mod changeable;
pub mod control;
pub mod font;
pub mod input;
pub mod visual;

use anyhow::{bail, Result};
use common::basic;
use once_cell::sync::Lazy;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

const INIT_ERR_TITLE: &str = "Rusted Ruins configulation error";

macro_rules! load_config_file {
    ($path:expr) => {{
        let path = cfg_path($path);
        info!("loading config file : \"{}\"", path.display());
        let s = match read_to_string(&path) {
            Ok(s) => s,
            Err(e) => crate::msg_box::exit_with_error(
                INIT_ERR_TITLE,
                format!("cannot load config file \"{}\"\n{}", path.display(), e),
            ),
        };

        match toml::de::from_str(&s) {
            Ok(config) => config,
            Err(e) => crate::msg_box::exit_with_error(
                INIT_ERR_TITLE,
                format!("cannot load config file \"{}\"\n{}", path.display(), e),
            ),
        }
    }};
}

/// Initialize lazy static
pub fn init() {
    Lazy::force(&ASSETS_DIR);
    Lazy::force(&USER_DIR);
    Lazy::force(&CONFIG);
    Lazy::force(&SCREEN_CFG);
    Lazy::force(&UI_CFG);
    Lazy::force(&INPUT_CFG);
    Lazy::force(&CONTROL_CFG);
    Lazy::force(&PAK_DIRS);
    changeable::initialize();
}

pub static ASSETS_DIR: Lazy<PathBuf> =
    Lazy::new(|| get_assets_dir().expect("cannot get data directory path"));
pub static USER_DIR: Lazy<PathBuf> = Lazy::new(get_user_dir);
pub static ADDON_DIR: Lazy<Option<PathBuf>> = Lazy::new(get_addon_dir);
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config: Config = match load_main_config_file() {
        Ok(config) => config,
        Err(e) => {
            error!("{}", e);
            crate::msg_box::exit_with_error(INIT_ERR_TITLE, e)
        }
    };
    args::modify_config_by_args(config)
});
pub static SCREEN_CFG: Lazy<visual::ScreenConfig> =
    Lazy::new(|| load_config_file!(&CONFIG.screen_config));
pub static UI_CFG: Lazy<visual::UiConfig> = Lazy::new(|| load_config_file!("ui.toml"));
pub static INPUT_CFG: Lazy<input::InputConfig> = Lazy::new(|| load_config_file!("input.toml"));
pub static CONTROL_CFG: Lazy<control::ControlConfig> =
    Lazy::new(|| load_config_file!("control.toml"));
pub static FONT_CFG: Lazy<font::FontConfig> = Lazy::new(|| load_config_file!("font.toml"));
pub static PAK_DIRS: Lazy<Vec<PathBuf>> = Lazy::new(|| vec![abs_path("paks")]);

/// Get application directory
fn get_assets_dir() -> Option<PathBuf> {
    if let Some(e) = env::var_os("RUSTED_RUINS_ASSETS_DIR") {
        return Some(PathBuf::from(e));
    }

    if cfg!(feature = "deb") {
        let path: PathBuf = "/usr/share/games/rusted-ruins/assets".into();
        if path.exists() {
            return Some(path);
        } else {
            warn!("not found assets directory \"{}\"", path.display());
        }
    }

    if let Ok(mut exe_file) = env::current_exe() {
        exe_file.pop();
        exe_file.push("assets");
        return Some(exe_file);
    }

    if let Ok(mut cdir) = env::current_dir() {
        cdir.push("assets");
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
    let mut v = vec![ASSETS_DIR.clone()];

    if ADDON_DIR.is_some() {
        v.push(ADDON_DIR.clone().unwrap());
    }

    v
}

/// Create absolute path from relative path which root is application directory
pub fn abs_path(s: &str) -> PathBuf {
    let mut path = ASSETS_DIR.clone();
    path.push(s);
    path
}

/// Create absolute path from config directory
pub fn cfg_path(s: &str) -> PathBuf {
    let mut path = USER_DIR.clone();
    path.push(basic::CFG_FILES_DIR);
    path.push(s);
    if path.exists() {
        return path;
    }

    let mut path = ASSETS_DIR.clone();
    path.push(basic::CFG_FILES_DIR);
    path.push(s);
    if !path.exists() {
        panic!("Config file {} does not exist", path.display());
    }
    path
}

/// Load main config file from user directory, or create it if not exist.
fn load_main_config_file() -> Result<Config> {
    let file_name = "config.toml";

    let mut path = USER_DIR.clone();
    path.push(basic::CFG_FILES_DIR);
    path.push(file_name);

    if !path.exists() {
        info!("config.toml not exist");

        let lang = crate::msg_box::lang_selector();

        let mut default_conf_path = ASSETS_DIR.clone();
        default_conf_path.push(basic::CFG_FILES_DIR);
        default_conf_path.push(file_name);

        let s = match read_to_string(&default_conf_path) {
            Ok(s) => s,
            Err(e) => {
                bail!("cannot read config file \"{}\"\n{}", path.display(), e);
            }
        };

        let mut default_config: Config = match toml::de::from_str(&s) {
            Ok(config) => config,
            Err(e) => {
                bail!(
                    "cannot load config file \"{}\"\n{}",
                    path.to_string_lossy(),
                    e
                );
            }
        };

        match lang {
            "ja" => {
                default_config.lang = "ja".into();
                default_config.second_lang = "en".into();
            }
            "en" => {
                default_config.lang = "en".into();
                default_config.second_lang = "ja".into();
            }
            _ => unreachable!(),
        }

        let config_string = toml::ser::to_string_pretty(&default_config).unwrap();

        let dir_path = path.parent().expect("invalid config path");
        if let Err(e) = std::fs::create_dir_all(dir_path) {
            bail!(
                "cannot create config directory \"{}\"\n{}",
                dir_path.to_string_lossy(),
                e
            );
        }

        info!("creating config file for {}", path.display());
        if let Err(e) = std::fs::write(&path, &config_string) {
            bail!(
                "cannot create config file \"{}\"\n{}",
                path.to_string_lossy(),
                e
            );
        }

        Ok(default_config)
    } else {
        let s = match read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                bail!("cannot load config file \"{}\"\n{}", path.display(), e);
            }
        };

        match toml::de::from_str(&s) {
            Ok(config) => Ok(config),
            Err(e) => {
                bail!("cannot load config file \"{}\"\n{}", path.display(), e);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CfgRect {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub lang: String,
    pub second_lang: String,
    pub screen_config: String,
    pub hardware_acceleration: bool,
    #[serde(default = "default_scale")]
    pub scale: i32,
    #[serde(default)]
    pub fix_rand: bool,
    pub enable_joystick: bool,
    pub sound_effect_volume: i32,
    pub music_volume: i32,
}

fn default_scale() -> i32 {
    1
}
