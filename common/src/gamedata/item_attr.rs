use crate::item_selector::ItemSelector;

use super::time::{Duration, Time};
use super::{item::*, UniqueId, UniqueIdGenerator};
use serde_with::{serde_as, DisplayFromStr};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

/// Attributes for ItemObject
#[serde_as]
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
    Container {
        #[serde_as(as = "DisplayFromStr")]
        selector: ItemSelector,
        /// Weight capacity (gram)
        capacity: u32,
        functions: Vec<ContainerFunction>,
    },
    /// Hours to rotting for food items
    Rot(u32),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ContainerFunction {
    PreventRot,
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
    pub(crate) item_list: ItemList,
    /// Id used for item ordering
    id: u64,
}

impl ItemListContainer {
    pub fn new<T: UniqueIdGenerator>(mut generator: T) -> Self {
        ItemListContainer {
            item_list: ItemList::default(),
            id: generator.generate(),
        }
    }

    pub fn id(&self) -> UniqueId {
        self.id
    }
}

impl AsRef<ItemList> for ItemListContainer {
    fn as_ref(&self) -> &ItemList {
        &self.item_list
    }
}

impl AsMut<ItemList> for ItemListContainer {
    fn as_mut(&mut self) -> &mut ItemList {
        &mut self.item_list
    }
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
