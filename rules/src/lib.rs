#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;
extern crate rusted_ruins_map_generator as map_generator;

pub mod active_skill;
pub mod biome;
pub mod chara;
pub mod charagen;
pub mod class;
pub mod combat;
pub mod creation;
pub mod dungeon_gen;
pub mod effect;
pub mod exp;
pub mod faction;
pub mod item;
pub mod magic;
pub mod map_gen;
pub mod material;
pub mod newgame;
pub mod npc_ai;
pub mod params;
pub mod quest;
pub mod race;
pub mod town;

use lazy_static::lazy_static;
use serde::de::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const ACTIVE_SKILL_DIR_NAME: &'static str = "active_skill";
const RECIPE_DIR_NAME: &'static str = "recipe";

/// Contain game rules
pub struct Rules {
    pub active_skills: active_skill::ActiveSkills,
    pub biome: biome::Biome,
    pub chara: chara::Chara,
    pub chara_gen: charagen::CharaGen,
    pub class: class::Classes,
    pub combat: combat::Combat,
    pub creation: creation::Creation,
    pub dungeon_gen: dungeon_gen::DungeonGen,
    pub exp: exp::Exp,
    pub effect: effect::Effect,
    pub faction: faction::Faction,
    pub map_gen: map_gen::MapGen,
    pub item: item::Item,
    pub magic: magic::Magic,
    pub material: material::Materials,
    pub newgame: newgame::NewGame,
    pub npc_ai: npc_ai::NpcAIs,
    pub params: params::Params,
    pub quest: quest::Quest,
    pub race: race::Races,
    pub town: town::Town,
}

impl Rules {
    fn load_from_dir(rules_dir: &Path, addon_dir: Option<&Path>) -> Rules {
        let mut dirs: Vec<PathBuf> = vec![rules_dir.into()];

        // Reading addon dirs
        if let Some(addon_dir) = addon_dir {
            fn read_addon_dir(dirs: &mut Vec<PathBuf>, addon_dir: &Path) -> std::io::Result<()> {
                let addon_dir = std::fs::read_dir(addon_dir)?;
                for d in addon_dir {
                    let d = d?;
                    if !d.file_type()?.is_dir() {
                        continue;
                    }
                    let mut rule_dir = d.path();
                    rule_dir.push("rules");
                    if rule_dir.exists() && rule_dir.is_dir() {
                        dirs.push(rule_dir);
                    }
                }
                Ok(())
            }
            if let Some(e) = read_addon_dir(&mut dirs, addon_dir).err() {
                error!("error occurred during reading rule dir: \"{}\"", e);
                exit_err();
            }
        }

        let mut active_skills = active_skill::ActiveSkills::default();
        let mut creation: creation::Creation = read_from_dirs(&dirs, "creation.json");

        for dir in dirs.iter() {
            let active_skill_dir = dir.join(ACTIVE_SKILL_DIR_NAME);
            if active_skill_dir.exists() {
                if let Err(e) = active_skills.join_from_dir(&active_skill_dir) {
                    warn!(
                        "active skill loading error at {}\n{}",
                        active_skill_dir.to_string_lossy(),
                        e
                    );
                }
            }
            let recipe_dir = dir.join(RECIPE_DIR_NAME);
            if recipe_dir.exists() {
                if let Err(e) = creation.join_from_dir(&recipe_dir) {
                    warn!(
                        "recipe loading error at {}\n{}",
                        recipe_dir.to_string_lossy(),
                        e
                    );
                }
            }
        }

        creation.sort();

        Rules {
            active_skills,
            biome: read_from_dirs(&dirs, "biome.json"),
            chara: read_from_dirs(&dirs, "chara.json"),
            chara_gen: read_from_dirs(&dirs, "charagen.json"),
            class: read_from_dirs(&dirs, "class.json"),
            creation,
            combat: read_from_dirs(&dirs, "combat.json"),
            dungeon_gen: read_from_dirs(&dirs, "dungeon_gen.json"),
            effect: read_from_dirs(&dirs, "effect.json"),
            exp: read_from_dirs(&dirs, "exp.json"),
            faction: read_from_dirs(&dirs, "faction.json"),
            map_gen: read_from_dirs(&dirs, "map_gen.json"),
            item: read_from_dirs(&dirs, "item.json"),
            magic: read_from_dirs(&dirs, "magic.json"),
            material: read_from_dirs(&dirs, "material.json"),
            newgame: read_from_dirs(&dirs, "newgame.json"),
            npc_ai: read_from_dirs(&dirs, "npc_ai.json"),
            params: read_from_dirs(&dirs, "params.json"),
            quest: read_from_dirs(&dirs, "quest.json"),
            race: read_from_dirs(&dirs, "race.json"),
            town: read_from_dirs(&dirs, "town.json"),
        }
    }
}

fn read_from_dirs<T, P>(dirs: &[P], name: &str) -> T
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut rule: Option<T> = None;

    for dir in dirs.iter() {
        let dir = dir.as_ref();
        let file_path = dir.join(name);
        let file = match fs::File::open(&file_path) {
            Ok(o) => o,
            Err(_) => {
                continue;
            }
        };
        info!("Rule file loading: \"{}\"", file_path.to_string_lossy());
        match serde_json::from_reader(file) {
            Ok(o) => {
                rule = Some(o);
            }
            Err(e) => {
                error!("{}", e);
                exit_err();
            }
        }
    }
    if let Some(rule) = rule {
        rule
    } else {
        error!("rule file \"{}\" not found", name);
        exit_err()
    }
}

fn read_from_file<T, P>(path: P) -> T
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    info!("Rule file loading: \"{}\"", path.as_ref().to_string_lossy());
    let file = match fs::File::open(path) {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            exit_err();
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
    static ref ADDON_RULES_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    /// Global state rules holder
    pub static ref RULES: Rules = Rules::load_from_dir(
        &RULES_DIR.lock().unwrap().as_ref().unwrap(),
        ADDON_RULES_DIR.lock().unwrap().as_ref().map(|path| path.as_ref()));
}

/// Initialize Rules
pub fn init<P: AsRef<Path>>(app_dirs: P, addon_dir: Option<P>) {
    *RULES_DIR.lock().unwrap() = Some(app_dirs.as_ref().join("rules"));

    if let Some(addon_dir) = addon_dir {
        *ADDON_RULES_DIR.lock().unwrap() = Some(addon_dir.as_ref().into());
    }

    lazy_static::initialize(&RULES);
}
