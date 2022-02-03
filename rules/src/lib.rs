#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;
extern crate rusted_ruins_map_generator as map_generator;

pub mod ability;
pub mod biome;
pub mod chara;
pub mod chara_trait;
pub mod charagen;
pub mod class;
pub mod combat;
pub mod creation;
pub mod dungeon_gen;
pub mod effect;
pub mod exp;
pub mod faction;
pub mod item;
pub mod map_gen;
pub mod material;
pub mod newgame;
pub mod npc;
pub mod npc_ai;
pub mod params;
pub mod power;
pub mod quest;
pub mod race;
pub mod recipe;
pub mod town;
pub mod world;

use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

trait Rule: DeserializeOwned {
    const NAME: &'static str;

    fn load<P: AsRef<Path>>(rule_dirs: &[P]) -> Result<Self> {
        info!("loading rule \"{}\"", Self::NAME);

        let mut rule: Option<Self> = None;
        for rule_dir in rule_dirs {
            let rule_dir = rule_dir.as_ref();
            let d = rule_dir.join(Self::NAME);

            if d.exists() && d.is_dir() {
                for entry in d.read_dir()? {
                    let rule_file = entry?.path();
                    let r = Self::from_file(&rule_file)
                        .with_context(|| format!("loading \"{}\"", rule_file.display()))?;
                    if let Some(rule) = rule.as_mut() {
                        rule.append(r);
                    } else {
                        rule = Some(r);
                    }
                }
            } else {
                let rule_file = rule_dir.join(format!("{}.ron", Self::NAME));
                if !rule_file.exists() {
                    continue;
                }

                let r = Self::from_file(&rule_file)
                    .with_context(|| format!("loading \"{}\"", rule_file.display()))?;
                if let Some(rule) = rule.as_mut() {
                    rule.append(r);
                } else {
                    rule = Some(r);
                }
            }
        }

        rule.ok_or_else(|| anyhow!("rule file not found for \"{}\" rule", Self::NAME))
    }

    fn from_file(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)?;
        let rule = ron::de::from_reader(file).unwrap();
        Ok(rule)
    }

    fn append(&mut self, other: Self);
}

/// Contain game rules
pub struct Rules {
    pub abilities: ability::Abilities,
    pub biomes: biome::Biomes,
    pub chara: chara::Chara,
    pub chara_gen: charagen::CharaGen,
    pub chara_traits: chara_trait::CharaTraits,
    pub classes: class::Classes,
    pub combat: combat::Combat,
    pub creation: creation::Creation,
    pub dungeon_gen: dungeon_gen::DungeonGen,
    pub exp: exp::Exp,
    pub effect: effect::Effect,
    pub faction: faction::Faction,
    pub map_gen: map_gen::MapGen,
    pub item: item::Item,
    pub materials: material::Materials,
    pub newgame: newgame::NewGame,
    pub npc: npc::Npc,
    pub npc_ai: npc_ai::NpcAIs,
    pub params: params::Params,
    pub power: power::Power,
    pub quest: quest::Quest,
    pub races: race::Races,
    pub recipes: recipe::Recipes,
    pub town: town::Town,
    pub world: world::World,
}

impl Rules {
    fn load_from_dir(rules_dir: &Path, addon_dir: Option<&Path>) -> anyhow::Result<Rules> {
        let mut dirs: Vec<PathBuf> = vec![rules_dir.into()];
        if let Some(addon_dir) = addon_dir {
            let addon_rule_dir = addon_dir.join("rules");
            dirs.push(addon_rule_dir);
        }

        Ok(Rules {
            abilities: ability::Abilities::load(&dirs)?,
            biomes: biome::Biomes::load(&dirs)?,
            chara: chara::Chara::load(&dirs)?,
            chara_gen: charagen::CharaGen::load(&dirs)?,
            chara_traits: chara_trait::CharaTraits::load(&dirs)?,
            classes: class::Classes::load(&dirs)?,
            creation: creation::Creation::load(&dirs)?,
            combat: combat::Combat::load(&dirs)?,
            dungeon_gen: dungeon_gen::DungeonGen::load(&dirs)?,
            effect: effect::Effect::load(&dirs)?,
            exp: exp::Exp::load(&dirs)?,
            faction: faction::Faction::load(&dirs)?,
            map_gen: map_gen::MapGen::load(&dirs)?,
            item: item::Item::load(&dirs)?,
            materials: material::Materials::load(&dirs)?,
            newgame: newgame::NewGame::load(&dirs)?,
            npc: npc::Npc::load(&dirs)?,
            npc_ai: npc_ai::NpcAIs::load(&dirs)?,
            params: params::Params::load(&dirs)?,
            power: power::Power::load(&dirs)?,
            quest: quest::Quest::load(&dirs)?,
            races: race::Races::load(&dirs)?,
            recipes: recipe::Recipes::load(&dirs)?,
            town: town::Town::load(&dirs)?,
            world: world::World::load(&dirs)?,
        })
    }
}

static RULES_DIR: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
static ADDON_RULES_DIR: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
/// Global state rules holder
pub static RULES: Lazy<Rules> = Lazy::new(|| {
    let rules = Rules::load_from_dir(
        RULES_DIR.lock().unwrap().as_ref().unwrap(),
        ADDON_RULES_DIR
            .lock()
            .unwrap()
            .as_ref()
            .map(|path| path.as_ref()),
    );

    match rules {
        Ok(rules) => rules,
        Err(e) => {
            error!("rules initalization failed:\n{}", e);
            std::process::exit(1);
        }
    }
});

/// Initialize Rules
pub fn init<P: AsRef<Path>>(app_dirs: P, addon_dir: Option<P>) {
    *RULES_DIR.lock().unwrap() = Some(app_dirs.as_ref().join("rules"));

    if let Some(addon_dir) = addon_dir {
        *ADDON_RULES_DIR.lock().unwrap() = Some(addon_dir.as_ref().into());
    }

    Lazy::force(&RULES);
}
