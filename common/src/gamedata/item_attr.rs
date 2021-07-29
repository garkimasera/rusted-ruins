use super::item::*;
use super::time::{Duration, Time};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

/// Attributes for ItemObject.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemObjAttr {
    /// Item nutrition
    Nutrition(u16),
    /// Ranged weapon animation
    AnimImgShot(String),
    /// Sound effect for weapons
    Sound(String),
    /// Range of charges
    Charge { min: u8, max: u8 },
    /// Plant data
    Plant {
        /// Time to harvestable
        growing_time_hours: u32,
        /// Reset time after harvesting. With None, the item is removed.
        reset_time_hours: Option<u32>,
        /// Required tile fertility to grow
        #[serde(default)]
        required_fertility: u8,
    },
    /// Hours to rotting for food items
    Rot(u32),
}

/// Items can have zero or more attributes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemAttr {
    /// Image variation
    ImageVariation(u32),
    /// Number of charges
    Charge { n: u32 },
    /// Data to generate the contents.
    /// Used to fix generated contents when this item is opened.
    ContentGen { level: u32, seed: u32 },
    /// Items in the container
    Container(ItemListContainer),
    /// Material of this item
    Material(MaterialName),
    /// For skill learning items
    SkillLearning(super::skill::SkillKind),
    /// Title for readable items
    Title(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemListContainer {
    /// Inner item list
    item_list: ItemList,
    /// Id used for item ordering
    id: u64,
}

impl PartialEq for ItemListContainer {
    fn eq(&self, other: &ItemListContainer) -> bool {
        self.id == other.id
    }
}

impl Eq for ItemListContainer {}

impl PartialOrd for ItemListContainer {
    fn partial_cmp(&self, other: &ItemListContainer) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for ItemListContainer {
    fn cmp(&self, other: &ItemListContainer) -> Ordering {
        self.id.cmp(&other.id)
    }
}
