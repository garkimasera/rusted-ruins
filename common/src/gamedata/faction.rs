use crate::basic::ARRAY_STR_ID_LEN;
use crate::hashmap::HashMap;
use arrayvec::ArrayString;

/// Faction information.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Faction {
    /// Faction relation to player.
    relation_table: HashMap<FactionId, FactionRelation>,
}

impl Default for Faction {
    fn default() -> Self {
        Faction {
            relation_table: HashMap::default(),
        }
    }
}

impl Faction {
    pub fn get(&self, faction: FactionId) -> FactionRelation {
        self.relation_table
            .get(&faction)
            .copied()
            .unwrap_or(FactionRelation(0))
    }

    pub fn set(&mut self, faction: FactionId, relation: FactionRelation) {
        self.relation_table.insert(faction, relation);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FactionId, &FactionRelation)> {
        self.relation_table.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactionId(arrayvec::ArrayString<ARRAY_STR_ID_LEN>);

impl Default for FactionId {
    fn default() -> FactionId {
        FactionId::unknown()
    }
}

impl FactionId {
    const PLAYER: &'static str = "!player";

    pub fn new(name: &str) -> Option<FactionId> {
        ArrayString::from(name).map(FactionId).ok()
    }

    pub fn unknown() -> FactionId {
        FactionId::new("!unknown").unwrap()
    }

    pub fn player() -> FactionId {
        FactionId::new(Self::PLAYER).unwrap()
    }

    pub fn is_player(&self) -> bool {
        self.as_str() == Self::PLAYER
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactionRelation(i16);
const FACTION_RELATION_MAX: i16 = 10000;
const FACTION_RELATION_MIN: i16 = -10000;

impl From<FactionRelation> for i16 {
    fn from(relation: FactionRelation) -> i16 {
        relation.0
    }
}

impl std::ops::Add<i16> for FactionRelation {
    type Output = Self;
    fn add(self, other: i16) -> Self {
        FactionRelation(std::cmp::min(
            self.0.wrapping_add(other),
            FACTION_RELATION_MAX,
        ))
    }
}

impl std::ops::Sub<i16> for FactionRelation {
    type Output = Self;
    fn sub(self, other: i16) -> Self {
        FactionRelation(std::cmp::max(
            self.0.wrapping_sub(other),
            FACTION_RELATION_MIN,
        ))
    }
}
