use super::DoPlayerAction;
use common::gamedata::*;
use common::gobj;
use geom::*;

impl<'a> DoPlayerAction<'a> {
    pub fn use_tool(&mut self, target: Vec2d) {
        let player = self.gd().chara.get(CharaId::Player);
        let tool = if let Some(tool) = player.equip.item(EquipSlotKind::Tool, 0) {
            tool
        } else {
            game_log_i!("use-tool-without-equip");
            return;
        };

        let item_obj = gobj::get_obj(tool.idx);

        match item_obj.tool_effect {
            ToolEffect::None => {
                warn!("try to use item that does not have any effect.");
            }
            ToolEffect::Build => {
                trace!("building at {}", &target);
                crate::game::building::start_build(self.0, target, CharaId::Player);
            }
        }
    }
}
