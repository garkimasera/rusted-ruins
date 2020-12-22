#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;

pub mod active_skill;
pub mod chara;
pub mod charagen;
pub mod class;
pub mod combat;
pub mod creation;
pub mod dungeon_gen;
pub mod effect;
pub mod exp;
pub mod faction;
pub mod floor_gen;
pub mod item;
pub mod magic;
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
    pub chara: chara::Chara,
    pub chara_gen: charagen::CharaGen,
    pub class: class::Classes,
    pub combat: combat::Combat,
    pub creation: creation::Creation,
    pub dungeon_gen: dungeon_gen::DungeonGen,
    pub exp: exp::Exp,
    pub effect: effect::Effect,
    pub faction: faction::Faction,
    pub floor_gen: floor_gen::FloorGen,
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
        let mut active_skills = active_skill::ActiveSkills::default();
        let active_skill_dir = rules_dir.join(ACTIVE_SKILL_DIR_NAME);
        if active_skill_dir.exists() {
            if let Err(e) = active_skills.join_from_dir(&active_skill_dir) {
                warn!(
                    "active skill loading error at {}\n{}",
                    active_skill_dir.to_string_lossy(),
                    e
                );
            }
        }

        let mut creation: creation::Creation = read_from_json(&rules_dir.join("creation.json"));
        let recipe_dir = rules_dir.join(RECIPE_DIR_NAME);
        if recipe_dir.exists() {
            if let Err(e) = creation.join_from_dir(&recipe_dir) {
                warn!(
                    "recipe loading error at {}\n{}",
                    recipe_dir.to_string_lossy(),
                    e
                );
            }
        }

        // Load active skill / creation(recipe) rule files in addon dir
        if let Some(addon_dir) = addon_dir {
            let addon_active_skill_dir = addon_dir.join(ACTIVE_SKILL_DIR_NAME);
            if addon_active_skill_dir.exists() {
                if let Err(e) = active_skills.join_from_dir(&addon_active_skill_dir) {
                    warn!(
                        "active skill loading error at {}\n{}",
                        addon_active_skill_dir.to_string_lossy(),
                        e
                    );
                }
            }

            let addon_recipe_dir = addon_dir.join(RECIPE_DIR_NAME);
            if addon_recipe_dir.exists() {
                if let Err(e) = creation.join_from_dir(&addon_recipe_dir) {
                    warn!(
                        "recipe loading error at {}\n{}",
                        addon_recipe_dir.to_string_lossy(),
                        e
                    );
                }
            }
        }

        creation.sort();

        Rules {
            active_skills,
            chara: read_from_json(&rules_dir.join("chara.json")),
            chara_gen: read_from_json(&rules_dir.join("charagen.json")),
            class: read_from_json(&rules_dir.join("class.json")),
            creation,
            combat: read_from_json(&rules_dir.join("combat.json")),
            dungeon_gen: read_from_json(&rules_dir.join("dungeon_gen.json")),
            effect: read_from_json(&rules_dir.join("effect.json")),
            exp: read_from_json(&rules_dir.join("exp.json")),
            faction: read_from_json(&rules_dir.join("faction.json")),
            floor_gen: read_from_json(&rules_dir.join("floor_gen.json")),
            item: read_from_json(&rules_dir.join("item.json")),
            magic: read_from_json(&rules_dir.join("magic.json")),
            material: read_from_json(&rules_dir.join("material.json")),
            newgame: read_from_json(&rules_dir.join("newgame.json")),
            npc_ai: read_from_json(&rules_dir.join("npc_ai.json")),
            params: read_from_json(&rules_dir.join("params.json")),
            quest: read_from_json(&rules_dir.join("quest.json")),
            race: read_from_json(&rules_dir.join("race.json")),
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
    static ref ADDON_RULES_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    /// Global state rules holder
    pub static ref RULES: Rules = Rules::load_from_dir(
        &RULES_DIR.lock().unwrap().as_ref().unwrap(),
        ADDON_RULES_DIR.lock().unwrap().as_ref().map(|path| path.as_ref()));
}

/// Initialize Rules
pub fn init<P: AsRef<Path>>(app_dirs: P, addon_dir: Option<P>) {
    let rules_dir = app_dirs.as_ref().join("rules");
    *RULES_DIR.lock().unwrap() = Some(rules_dir);

    if let Some(addon_dir) = addon_dir {
        let addon_rules_dir = addon_dir.as_ref().join("rules");
        *ADDON_RULES_DIR.lock().unwrap() = Some(addon_rules_dir);
    }

    lazy_static::initialize(&RULES);
}
