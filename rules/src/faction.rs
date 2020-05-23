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

    pub fn name_to_faction(&self, faction_name: &str) -> common::gamedata::FactionId {
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
        common::gamedata::FactionId(i as u8)
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct FactionInfo {
    pub name: String,
    pub default_relation: FactionRelation,
}
