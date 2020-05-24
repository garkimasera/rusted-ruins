/// Faction information.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Faction {
    /// Faction relation to player.
    relation_table: Vec<FactionRelation>,
}

impl Faction {
    pub fn new() -> Faction {
        Faction {
            relation_table: vec![FactionRelation(0); 0xFF],
        }
    }

    pub fn get(&self, faction: FactionId) -> FactionRelation {
        self.relation_table[faction.0 as usize]
    }

    pub fn set(&mut self, faction: FactionId, relation: FactionRelation) {
        self.relation_table[faction.0 as usize] = relation;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct FactionId(pub u8);

impl Default for FactionId {
    fn default() -> FactionId {
        FactionId::UNKNOWN
    }
}

impl FactionId {
    pub const UNKNOWN: FactionId = FactionId(0);
    pub const PLAYER: FactionId = FactionId(1);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct FactionRelation(i8);
const FACTION_RELATION_MAX: i8 = 100;
const FACTION_RELATION_MIN: i8 = -100;

impl std::ops::Add<i8> for FactionRelation {
    type Output = Self;
    fn add(self, other: i8) -> Self {
        FactionRelation(std::cmp::min(
            self.0.wrapping_add(other),
            FACTION_RELATION_MAX,
        ))
    }
}

impl std::ops::Sub<i8> for FactionRelation {
    type Output = Self;
    fn sub(self, other: i8) -> Self {
        FactionRelation(std::cmp::max(
            self.0.wrapping_sub(other),
            FACTION_RELATION_MIN,
        ))
    }
}
