use super::extrait::*;
use super::play_time::UniqueIdGeneratorByTime;
use common::gamedata::*;
use common::gobj;
use common::objholder::CharaTemplateIdx;
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

    fn add_chara_to_party(&mut self, mut chara: Chara) -> bool {
        if !self.has_empty_for_party() {
            return false;
        }

        let cid = CharaId::Ally {
            id: UniqueIdGeneratorByTime.generate(),
        };
        chara.faction = FactionId::player();

        if !self.player.party.insert(cid) {
            return false;
        }

        self.add_chara(cid, chara);

        let player_pos = self.player_pos();
        let map = self.get_current_map_mut();
        if let Some(pos) = map.empty_tile_around(player_pos) {
            map.locate_chara(cid, pos);
        }
        trace!("added new chara to the player's party");

        true
    }

    fn gen_party_chara(&mut self, id: &str, lv: u32) -> bool {
        trace!("generating party chara \"{}\" lv.{}", id, lv);
        let idx: CharaTemplateIdx = gobj::id_to_idx(id);
        let chara = crate::game::chara::gen::create_chara(idx, lv, FactionId::player(), None);
        self.add_chara_to_party(chara)
    }
}
