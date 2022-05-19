use crate::Rule;
use common::gamedata::*;
use common::gobj::ObjIdxAsId;
use common::objholder::ItemIdx;
use geom::Coords;
use serde_with::serde_as;
use std::collections::HashMap;

/// Rules for starting new game
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct NewGame {
    /// The choices of character class
    pub class_choices: Vec<CharaClass>,
    /// The choices of character trait
    pub trait_choices: Vec<String>,
    pub trait_initial_point: i32,
    pub start_region: String,
    pub start_pos: Coords,
    pub start_money: u32,
    pub chara_template_table: HashMap<CharaClass, String>,
    pub common_initial_skills: Vec<SkillKind>,
    pub common_initial_abilities: Vec<AbilityId>,
    #[serde_as(as = "Vec<(ObjIdxAsId, _)>")]
    pub common_initial_items: Vec<(ItemIdx, u32)>,
    /// Initial game date (year)
    pub initial_date_year: u32,
    /// Initial game date (month)
    pub initial_date_month: u32,
    /// Initial game date (day)
    pub initial_date_day: u32,
    /// Initial game date (hour)
    pub initial_date_hour: u32,
}

impl Rule for NewGame {
    const NAME: &'static str = "newgame";
}
