use super::skill::SkillKind;
use derivative::Derivative;
use fnv::FnvHashMap;

/// Represents passive effect for character traits, items, etc.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Property {
    CharaStr(i16),
    CharaVit(i16),
    CharaDex(i16),
    CharaInt(i16),
    CharaWil(i16),
    CharaCha(i16),
    CharaSpd(i16),
}

/// Total passive effect a character received by properties, status, and other factors.
#[derive(Clone, Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct CharaTotalEffect {
    pub base_hp: i32,
    pub max_hp: i32,
    pub str: i16,
    pub vit: i16,
    pub dex: i16,
    pub int: i16,
    pub wil: i16,
    pub cha: i16,
    pub spd: i16,
    #[derivative(Default(value = "1.0"))]
    pub spd_factor: f32,
    pub skill_level: FnvHashMap<SkillKind, (f32, i32)>,
}
