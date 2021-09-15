use super::extrait::*;
use common::gamedata::*;
use rules::RULES;

#[extend::ext(pub, name = GameDataPartyExt)]
impl GameData {
    /// The maximum party size
    fn available_party_size(&self) -> u32 {
        let player = self.chara.get(CharaId::Player);
        let skill_level = player.skill_level(SkillKind::Leadership);
        let cha = player.attr.cha as u32;
        (cha / 10 + skill_level / 10 + 1).clamp(1, RULES.npc.party_size_max)
    }

    fn has_empty_for_party(&self) -> bool {
        (self.player.party.len() as u32 + 1) < self.available_party_size()
    }
}
