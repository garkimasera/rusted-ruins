use super::DoPlayerAction;
use crate::game::effect::do_effect;
use crate::game::extrait::*;
use crate::game::{Animation, InfoGetter};
use common::gamedata::*;
use common::gobj;
use common::objholder::AnimImgIdx;
use geom::*;
use once_cell::sync::Lazy;
use rules::RULES;
use CharaId::Player;

static MINING_ANIM_IDX: Lazy<AnimImgIdx> = Lazy::new(|| gobj::id_to_idx("mining"));

impl<'a, 's> DoPlayerAction<'a, 's> {
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
        let tool_effect = if let Some(tool_effect) = item_obj.tool_effect.as_ref() {
            tool_effect
        } else {
            warn!("try to use item that does not have any effect.");
            return;
        };

        match tool_effect {
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
                let skill_level = player.skills.get(SkillKind::Mining);
                let power = skill_level as f32 * RULES.effect.mining_power_factor
                    + RULES.effect.mining_power_base;
                do_effect(self.0, &effect, Some(CharaId::Player), pos, power, 0.0);
                let floor_level = self.0.gd.get_current_mapid().floor();
                let player = self.0.gd.chara.get_mut(CharaId::Player);
                player
                    .skills
                    .add_exp(SkillKind::Mining, RULES.exp.mining, floor_level);
                self.0
                    .anim_queue
                    .push(Animation::img_onetile(*MINING_ANIM_IDX, pos));
                audio::play_sound("mining");
                self.0.finish_player_turn();
            }
        }
    }
}
