
pub mod creation;

use super::Game;
use super::combat::DamageKind;
use common::gamedata::GameData;
use common::gamedata::chara::{Chara, CharaId};

pub fn damage(game: &mut Game, target: CharaId, damage: i32, damage_kind: DamageKind) {
    let t = game.gd.chara.get_mut(target);
    
    t.hp -= damage;

    if t.hp < 0 {
        game.dying_charas.push(target);
        // Logging
        match damage_kind {
            DamageKind::CloseRangeAttack => {
                game_log!("killed-by-close-attack"; target=t.name);
            },
        }
    }
}

pub fn update_params_by_id(gd: &mut GameData, cid: CharaId) {
    update_params(gd.chara.get_mut(cid));
}

pub fn update_params(chara: &mut Chara) {
    chara.params.max_hp = chara.base_params.max_hp;
    chara.params.vit = chara.base_params.vit;
    chara.params.dex = chara.base_params.dex;
    chara.params.int = chara.base_params.int;
    chara.params.wil = chara.base_params.wil;
    chara.params.cha = chara.base_params.cha;
    chara.params.spd = chara.base_params.spd;
}

