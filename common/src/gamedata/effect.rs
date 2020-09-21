use super::defs::Element;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Effect {
    pub kind: Vec<EffectKind>,
    #[serde(default)]
    pub power_adjust: Vec<f32>,
}

/// Effect defines the game effect of items, magics, or other active skills.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectKind {
    None,
    RecoverHp,
    RecoverSp,
    RecoverMp,
    Melee { element: Element },
    Ranged { element: Element },
    Explosion { element: Element },
    Direct { element: Element },
    Status { status: StatusEffect },
    CharaScan,
}

impl Default for EffectKind {
    fn default() -> Self {
        EffectKind::None
    }
}

/// Effect defines the game effect of items, magics, or other active skills.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusEffect {
    Asleep,
    Poison,
}
