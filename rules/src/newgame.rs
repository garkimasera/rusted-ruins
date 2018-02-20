
use std::collections::HashMap;
use array2d::Vec2d;
use common::gamedata::chara::CharaClass;

/// Rules for starting new game
#[derive(Serialize, Deserialize)]
pub struct NewGame {
    /// The choices of character class
    pub class_choices: Vec<CharaClass>,
    pub start_region: String,
    pub start_pos: Vec2d,
    pub chara_template_table: HashMap<CharaClass, String>,
}

