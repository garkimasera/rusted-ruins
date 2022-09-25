use super::ASSETS_DIR;
use once_cell::sync::Lazy;
use std::fs::read_to_string;
use std::process::exit;
use std::sync::{RwLock, RwLockReadGuard};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ChangeableConfig {
    pub game_log: GameLogConfig,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct GameLogConfig {
    pub combat_log: CombatLog,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatLog {
    None = 0,
    Minimum,
    Detail,
}

impl CombatLog {
    pub fn damage(&self) -> bool {
        *self == CombatLog::Detail
    }

    pub fn attack(&self) -> bool {
        *self == CombatLog::Detail
    }
}

static CHANGEABLE_CFG: Lazy<RwLock<ChangeableConfig>> =
    Lazy::new(|| RwLock::new(load_changeable_cfg()));

pub fn initialize() {
    Lazy::force(&CHANGEABLE_CFG);
}

pub fn read() -> RwLockReadGuard<'static, ChangeableConfig> {
    CHANGEABLE_CFG.read().expect("config read")
}

pub fn game_log_cfg() -> GameLogConfig {
    read().game_log
}

fn load_changeable_cfg() -> ChangeableConfig {
    let mut path = ASSETS_DIR.clone();
    path.push(common::basic::CFG_FILES_DIR);
    path.push("changeable.default.toml");
    let s = match read_to_string(&path) {
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
}
