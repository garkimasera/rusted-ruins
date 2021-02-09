//! Miscellaneous type definitions

use crate::gamedata::effect::Effect;
use crate::objholder::ItemIdx;
use std::ops::{Index, IndexMut};

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    None = -1,
    Physical = 0,
    Fire = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

pub const ELEMENTS: [Element; Element::Spirit as usize + 1] = [
    Element::Physical,
    Element::Fire,
    Element::Cold,
    Element::Shock,
    Element::Poison,
    Element::Spirit,
];

/// This array has the same size as element types.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct ElementArray<T>(pub [T; Element::Spirit as usize + 1]);

impl<T> Index<Element> for ElementArray<T> {
    type Output = T;
    fn index(&self, e: Element) -> &T {
        assert_ne!(e, Element::None);
        &self.0[e as usize]
    }
}

impl<T> IndexMut<Element> for ElementArray<T> {
    fn index_mut(&mut self, e: Element) -> &mut T {
        assert_ne!(e, Element::None);
        &mut self.0[e as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ElementProtection(i8);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillBonus {
    None,
    VeryLow,
    Low,
    Normal,
    High,
    VeryHigh,
    Superb,
}

impl Default for SkillBonus {
    fn default() -> SkillBonus {
        SkillBonus::None
    }
}

/// A recipe for creation
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub product: String,
    pub ingredients: Vec<(String, u32)>,
    pub facility: Option<String>,
    pub difficulty: u32,
    pub required_time: CreationRequiredTime,
    #[serde(default)]
    pub put_on_ground: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreationRequiredTime {
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolEffect {
    Build,
    Chop,
    Mine,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum UseEffect {
    Effect(Effect),
    Deed,
}

/// Reward for quests or events
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct Reward {
    pub money: i64,
    pub item: Vec<ItemIdx>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Harvest {
    pub harvest_type: HarvestType,
    pub target_item: String,
    pub difficulty: u32,
    pub n_yield: u32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarvestType {
    Animal,
    Chop,
    Crop,
    Deconstruct,
    Mine,
}
