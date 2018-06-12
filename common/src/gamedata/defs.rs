//! Miscellaneous type definitions

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Element {
    None = -1,
    Physical = 0,
    Fire = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

