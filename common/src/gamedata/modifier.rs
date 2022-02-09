use super::{skill::SkillKind, Element, ElementArray};
use derivative::Derivative;
use fnv::FnvHashMap;
use ordered_float::NotNan;

/// Represents modifier for character.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CharaModifier {
    Str(i16),
    Vit(i16),
    Dex(i16),
    Int(i16),
    Wil(i16),
    Cha(i16),
    Spd(i16),
    Defence {
        element: Element,
        value: NotNan<f32>,
    },
    DefenceMultiplier {
        element: Element,
        value: NotNan<f32>,
    },
}

/// Summed effect of modifiers a character received by properties, status, and other factors.
#[derive(Clone, Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct CharaTotalModifier {
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
    pub defence: ElementArray<(f32, f32)>,
}
