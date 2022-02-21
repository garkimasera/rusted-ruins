//! Miscellaneous type definitions

use ordered_float::NotNan;

use crate::gamedata::effect::Effect;
use crate::gamedata::skill::{MagicKind, SkillKind, WeaponKind};
use crate::objholder::ItemIdx;
use enum_map::Enum;
use std::ops::{Index, IndexMut};

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Element {
    None = -1,
    Physical = 0,
    Heat = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

pub const ELEMENTS: [Element; Element::Spirit as usize + 1] = [
    Element::Physical,
    Element::Heat,
    Element::Cold,
    Element::Shock,
    Element::Poison,
    Element::Spirit,
];

/// This array has the same size as element types.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Enum)]
pub enum AttackKind {
    Melee,
    Ranged,
    Explosion,
    Direct,
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
pub enum CreationRequiredTime {
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ToolEffect {
    Build,
    Chop,
    Mine,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum UseEffect {
    Effect(Effect),
    Deed,
    Seed { plant: String },
    SelectBuilding,
    InsertModule,
}

/// Reward for quests or events
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct Reward {
    #[serde(default)]
    pub money: i64,
    #[serde(default)]
    pub items: Vec<(ItemIdx, u32)>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Harvest {
    pub kind: HarvestKind,
    /// item id and yield
    pub items: Vec<(String, u32, u32)>,
    pub difficulty: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum HarvestKind {
    Animal,
    Chop,
    Deconstruct,
    Plant,
    Mine,
}

impl HarvestKind {
    pub fn related_skill(self) -> SkillKind {
        match self {
            HarvestKind::Animal => SkillKind::Animals,
            HarvestKind::Chop => SkillKind::Plants,
            HarvestKind::Deconstruct => SkillKind::Construction,
            HarvestKind::Plant => SkillKind::Plants,
            HarvestKind::Mine => SkillKind::Mining,
        }
    }
}

/// Ability id.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AbilityId(pub String);

impl std::fmt::Display for AbilityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    pub group: AbilityGroup,
    pub icon: String,
    pub effect: Effect,
    pub power_calc: PowerCalcMethod,
    #[serde(default)]
    pub cost_sp: u32,
    #[serde(default)]
    pub cost_mp: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PowerCalcMethod {
    Fixed,
    BareHands,
    Melee(WeaponKind),
    Ranged(WeaponKind),
    Release,
    Magic(MagicKind),
    Medical,
    Throw(u32),
    Custom(String),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct BasePower(pub NotNan<f32>, pub NotNan<f32>);

impl Default for BasePower {
    fn default() -> Self {
        BasePower::new(1.0, 0.0)
    }
}

impl BasePower {
    pub fn new(p: f32, var: f32) -> Self {
        BasePower(NotNan::new(p).unwrap(), NotNan::new(var).unwrap())
    }
}

impl std::ops::Mul<f32> for BasePower {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let rhs = NotNan::new(rhs).unwrap();
        BasePower(self.0 * rhs, self.1)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AbilityGroup {
    Special,
    Magic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum AbilityOrigin {
    Inherent,
    Learned,
    Race,
    Class,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum PassiveEffect {
    AttrStr(i16),
    AttrVit(i16),
    AttrDex(i16),
    AttrInt(i16),
    AttrWil(i16),
    AttrCha(i16),
    AttrSpd(i16),
}

/// Unique id for used in game
pub type UniqueId = u64;

/// Unique id generator
pub trait UniqueIdGenerator {
    fn generate(&mut self) -> UniqueId;
}
