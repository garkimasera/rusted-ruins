use super::DoPlayerAction;
use common::gamedata::*;
use rules::RULES;

impl<'a> DoPlayerAction<'a> {
    pub fn restart(&mut self) {
        let gd = self.gd_mut();
        let player = gd.chara.get_mut(CharaId::Player);
        player.hp = player.attr.max_hp;

        let (mid, pos) = gd
            .region
            .path_to_map_id_and_pos(&RULES.world.restart_path)
            .unwrap();
        crate::game::map::switch_map(self.0, Destination::MapIdWithPos(mid, pos));
        self.0.start_script(&RULES.world.restart_script, None, None);
    }
}
