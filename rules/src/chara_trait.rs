use common::gamedata::Property;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharaTraits(HashMap<String, CharaTrait>);

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct CharaTrait {
    cost: i32,
    properties: Vec<Property>,
}
