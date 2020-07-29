use super::defs::Element;

/// Effect defines the game effect of items, magics, or other active skills.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    RecoverHp,
    RecoverSp,
    RecoverMp,
    Melee(Element),
    Ranged(Element),
    Explosion(Element),
    Direct(Element),
    CharaScan,
}
