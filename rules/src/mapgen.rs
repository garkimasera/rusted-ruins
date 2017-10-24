
use std::collections::HashMap;
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::Race;
use rand::{Rng, thread_rng};

/// Rules for map generation
#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub npc_gen: HashMap<DungeonKind, HashMap<Race, f32>>,
}

impl MapGen {
    pub fn new() -> MapGen {
        MapGen {
            npc_gen: HashMap::new(),
        }
    }

    /// Choose one race from npc_gen map according to the weights
    pub fn choose_race(&self, dungeon_kind: DungeonKind) -> Race {
        let npc_gen = self.npc_gen.get(&dungeon_kind).expect(
            &format!("Internal error: {:?} is not specified for npc generation rule", dungeon_kind));
        assert!(npc_gen.len() > 0);
        
        let mut sum_w = npc_gen.values().sum();
        let mut rng = thread_rng();
        
        for (race, weight) in npc_gen {
            if rng.gen_range(0.0, sum_w) <= *weight {
                return *race;
            }
            sum_w -= *weight;
        }
        *npc_gen.keys().next().unwrap()
    }
}

