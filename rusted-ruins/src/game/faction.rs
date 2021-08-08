use common::gamedata::*;
use rules::RULES;

#[extend::ext(pub)]
impl Faction {
    fn change(&mut self, faction: FactionId, value: i16) {
        if let Some(true) = RULES
            .faction
            .factions
            .get(&faction)
            .map(|faction| faction.constant)
        {
            return;
        }

        let current_value = self.get(faction);
        self.set(faction, current_value + value);

        if value > 0 {
            game_log_i!("faction-relation-improve"; faction=faction, value=value);
        } else if value < 0 {
            let value = -value;
            game_log_i!("faction-relation-lower"; faction=faction, value=value);
        }
    }
}
