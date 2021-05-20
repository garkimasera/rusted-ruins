use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Faction {
    pub relation_friend: FactionRelation,
    pub relation_neutral: FactionRelation,
    pub factions: HashMap<FactionId, FactionInfo>,
}

impl Faction {
    pub fn get(&self, faction: FactionId) -> &FactionInfo {
        self.factions
            .get(&faction)
            .unwrap_or(&self.factions[&FactionId::default()])
    }

    pub fn relation(&self, f1: FactionId, f2: FactionId) -> FactionRelation {
        let f1 = self.get(f1);
        let f2 = self.get(f2);
        std::cmp::min(f1.default_relation, f2.default_relation)
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct FactionInfo {
    pub default_relation: FactionRelation,
}
