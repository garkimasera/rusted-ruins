//! Miscellaneous type definitions

/// Elements of damage/attack
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Element {
    Physical, Fire, Cold, Shock, Poison, Spirit, AntiMagic,
}

