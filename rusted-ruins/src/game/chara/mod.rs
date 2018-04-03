
pub mod creation;
pub mod preturn;
pub mod status;

use super::Game;
use super::combat::DamageKind;
use common::gamedata::GameData;
use common::gamedata::chara::{Chara, CharaId};
use common::gobj;
use rules::RULES;

/// Additional Chara method
pub trait CharaEx {
    fn get_name(&self) -> &str;
}

impl CharaEx for Chara {
    fn get_name(&self) -> &str {
        if let Some(ref name) = self.name {
            name
        } else {
            ::text::obj_txt(gobj::idx_to_id(self.template))
        }
    }
}

pub fn damage(game: &mut Game, cid: CharaId, damage: i32, damage_kind: DamageKind) {
    let chara = game.gd.chara.get_mut(cid);
    
    chara.hp -= damage;

    if chara.hp < 0 {
        game.dying_charas.push(cid);
        // Logging
        match damage_kind {
            DamageKind::ShortRangeAttack => {
                game_log!("killed-by-short-range-attack"; chara=chara.get_name());
            }
            DamageKind::Poison => {
                game_log!("killed-by-poison-damage"; chara=chara.get_name());
            }
        }
    }
}

pub fn update_params_by_id(gd: &mut GameData, cid: CharaId) {
    update_params(gd.chara.get_mut(cid));
}

pub fn update_params(chara: &mut Chara) {
    chara.params.max_hp = chara.base_params.max_hp;
    chara.params.str = chara.base_params.str;
    chara.params.vit = chara.base_params.vit;
    chara.params.dex = chara.base_params.dex;
    chara.params.int = chara.base_params.int;
    chara.params.wil = chara.base_params.wil;
    chara.params.cha = chara.base_params.cha;
    chara.params.spd = chara.base_params.spd;
    chara.params.view_range = RULES.chara.default_view_range;
}

