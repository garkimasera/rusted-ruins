use super::DoPlayerAction;
use crate::game::effect::do_effect;
use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use geom::*;
use CharaId::Player;

impl<'a> DoPlayerAction<'a> {
    pub fn use_tool(&mut self, pos: Vec2d) {
        let player = self.gd().chara.get(CharaId::Player);
        let player_pos = self.gd().player_pos();
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
                if !pos.is_adjacent(player_pos) {
                    game_log_i!("building-not-adjacent-tile");
                    return;
                }
                trace!("building at {}", &pos);
                crate::game::building::start_build(self.0, pos, Player);
            }
            ToolEffect::Chop => {
                trace!("chopping at {}", &pos);
                crate::game::action::harvest::harvest_by_tool(self.0, CharaId::Player, pos);
                self.0.finish_player_turn();
            }
            ToolEffect::Mine => {
                let map = self.0.gd.get_current_map();
                if map.tile[pos].wall.is_empty() {
                    return;
                }
                if !pos.is_adjacent(player_pos) {
                    game_log_i!("mining-not-adjacent-tile");
                    return;
                }

                trace!("mining at {}", &pos);
                let effect = Effect {
                    kind: vec![EffectKind::WallDamage],
                    ..Effect::default()
                };
                do_effect(self.0, &effect, Some(CharaId::Player), pos, 1.0, 0.0);
                self.0.finish_player_turn();
            }
        }
    }
}
