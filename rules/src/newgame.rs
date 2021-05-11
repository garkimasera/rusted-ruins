use common::gamedata::*;
use geom::Vec2d;
use std::collections::HashMap;

/// Rules for starting new game
#[derive(Serialize, Deserialize)]
pub struct NewGame {
    /// The choices of character class
    pub class_choices: Vec<CharaClass>,
    pub start_region: String,
    pub start_pos: Vec2d,
    pub start_money: u32,
    pub chara_template_table: HashMap<CharaClass, String>,
    pub common_initial_skills: Vec<SkillKind>,
    /// Initial game date (year)
    pub initial_date_year: u32,
    /// Initial game date (month)
    pub initial_date_month: u32,
    /// Initial game date (day)
    pub initial_date_day: u32,
    /// Initial game date (hour)
    pub initial_date_hour: u32,
}
