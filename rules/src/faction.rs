use common::gamedata::*;

#[derive(Serialize, Deserialize)]
pub struct Faction {
    pub relation_friend: FactionRelation,
    pub relation_neutral: FactionRelation,
    pub factions: Vec<FactionInfo>,
}

impl Faction {
    pub fn get(&self, faction: common::gamedata::FactionId) -> &FactionInfo {
        self.factions
            .get(faction.0 as usize)
            .unwrap_or(&self.factions[0])
    }

    pub fn get_by_name(&self, faction_name: &str) -> &FactionInfo {
        self.factions
            .iter()
            .find(|faction| faction.name == faction_name)
            .unwrap_or(&self.factions[0])
    }

    pub fn name_to_faction(&self, faction_name: &str) -> FactionId {
        let i = self
            .factions
            .iter()
            .enumerate()
            .find_map(|(i, faction)| {
                if faction.name == faction_name {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap_or(0);
        FactionId(i as u8)
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
    pub name: String,
    pub default_relation: FactionRelation,
}
