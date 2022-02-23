use ordered_float::NotNan;

use crate::item_selector::ItemSelector;
use crate::objholder::ItemIdx;

use super::effect::Effect;
use super::time::{Duration, Time};
use super::{defs::*, CharaModifier};
use super::{item::*, BasePower, UniqueId, UniqueIdGenerator};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

/// Attributes for ItemObject
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemObjAttr {
    /// Item nutrition
    Nutrition(u16),
    /// Ranged weapon animation
    AnimImgShot(String),
    /// Sound effect for weapons
    Sound(String),
    /// Range of charges
    Charge {
        min: u8,
        max: u8,
    },
    /// Plant data
    Plant {
        /// Duration to harvestable
        #[serde(default)]
        growing_duration: Duration,
        /// Reset time after harvesting used for repeated cultivation. With None, the item is removed.
        repeat_duration: Option<Duration>,
        /// Required tile fertility to grow
        #[serde(default)]
        required_fertility: u8,
    },
    Container {
        selector: ItemSelector,
        /// Weight capacity (gram)
        capacity: u32,
        #[serde(default)]
        prevent_rot: bool,
        #[serde(default)]
        function: ContainerFunction,
    },
    /// Hours to rotting for food items
    Rot(Duration),
    /// Power for weapon items
    Weapon {
        base_power: BasePower,
        hit: NotNan<f32>,
    },
    /// Defence for armor items
    Defence(ElementArray<u16>),
    /// Effect for food or potion items
    Medical {
        effect: Effect,
    },
    /// Effect for releasable items
    Release {
        effect: Effect,
    },
    /// Effect for throwable items
    Throw {
        effect: Effect,
    },
    /// For harvestable items
    Harvest {
        kind: HarvestKind,
        items: Vec<(String, u32, u32)>,
        difficulty: u32,
    },
    /// For tool items
    Tool(ToolEffect),
    /// For usable items
    Use(UseEffect),
    /// Facility type for creation and additional quality.
    Facility {
        #[serde(rename = "type")]
        ty: String,
        quality: i8,
    },
    /// Module effects with random choose weight
    Module {
        effects: Vec<(ModuleEffect, NotNan<f32>)>,
    },
    CharaModifier(CharaModifier),
    Titles(Vec<String>),
    ConvertableByContainer {
        kind: String,
        products: Vec<(String, u32)>,
        #[serde(default = "Duration::zero")]
        duration: Duration,
    },
}

impl ItemObjAttr {
    pub fn harvest(&self) -> Option<Harvest> {
        match self {
            ItemObjAttr::Harvest {
                kind,
                items,
                difficulty,
            } => Some(Harvest {
                kind: *kind,
                items: items.clone(),
                difficulty: *difficulty,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ContainerFunction {
    None,
    DeliveryChest,
    Converter {
        kind: String,
    },
    ConvertMixed {
        #[serde(default = "Duration::zero")]
        duration: Duration,
        product: String,
        #[serde(default = "product_multiplier_default")]
        product_multiplier: u32,
        ingredients: Vec<(ItemSelector, u32)>,
    },
}

impl Default for ContainerFunction {
    fn default() -> Self {
        ContainerFunction::None
    }
}

fn product_multiplier_default() -> u32 {
    1
}

/// Items can have zero or more attributes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemAttr {
    /// Image variation
    ImageVariation(u32),
    /// Number of charges
    Charge {
        n: u32,
    },
    /// Data to generate the contents.
    /// Used to fix generated contents when this item is opened.
    ContentGen {
        level: u32,
        seed: u32,
    },
    /// Items in the container
    Container(ItemListContainer),
    /// Material of this item
    Material(MaterialName),
    /// For module items
    Module(ModuleEffect),
    /// For weapon/armor items
    ModuleSlot {
        kind: ModuleSlotKind,
        content: Option<(ItemIdx, ModuleEffect)>,
    },
    /// For skill learning items
    SkillLearning(super::skill::SkillKind),
    /// Title for readable items
    Title(String),
    BuildObj(BuildObj),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum BuildObj {
    Tile(String),
    Wall(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemListContainer {
    /// Inner item list
    pub item_list: ItemList,
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

    pub fn item_list(&self) -> &ItemList {
        &self.item_list
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum ModuleSlotKind {
    Ability,
    Core,
    Extend,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ModuleEffect {
    Ability { group: String },
    Extend(ExtendModuleEffect),
    Core,
}

impl ModuleEffect {
    pub fn kind(&self) -> ModuleSlotKind {
        match self {
            Self::Ability { .. } => ModuleSlotKind::Ability,
            Self::Extend(_) => ModuleSlotKind::Extend,
            Self::Core => ModuleSlotKind::Core,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ExtendModuleEffect {
    Chara(CharaModifier),
    Weapon(WeaponModifier),
}

/// Represents modifier for weapon item
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum WeaponModifier {
    PowerFactor(NotNan<f32>),
}
