
//! Process characters action

use super::Game;
use super::combat;
use super::extrait::*;
use array2d::*;
use common::gamedata::GameData;
use common::gamedata::chara::*;
use common::gamedata::item::*;
use common::gobj;

pub fn try_move(game: &mut Game, chara_id: CharaId, dir: Direction) -> bool {
    if dir.as_vec() == (0, 0) { return true; } // Move to current tile always success
    let dest_tile = game.gd.get_current_map().chara_pos(chara_id).unwrap() + dir.as_vec();

    if !game.gd.get_current_map().is_movable(dest_tile) {
        return false;
    }

    let other_chara = game.gd.get_current_map().get_chara(dest_tile);
    if other_chara.is_none() {
        game.gd.get_current_map_mut().move_chara(chara_id, dir);
        if chara_id == CharaId::Player {
            game.anim_queue.push_player_move(dir);
        }
    }else{
        let rel = {
            let chara = game.gd.chara.get(chara_id);
            let other_chara = game.gd.chara.get(other_chara.unwrap());
            chara.rel.relative(other_chara.rel)
        };
        match rel {
            Relationship::ALLY | Relationship::FRIENDLY |Relationship::NEUTRAL => {
                {
                    let current_map = game.gd.get_current_map_mut();
                    current_map.move_chara(chara_id, dir);
                }
                if chara_id == CharaId::Player {
                    game.anim_queue.push_player_move(dir);
                }
            },
            Relationship::HOSTILE => {
                combat::attack_neighbor(game, chara_id, other_chara.unwrap());
            },
        }
    }
    true
}

/// Drink one item
pub fn drink_item(gd: &mut GameData, il: ItemLocation, cid: CharaId) {
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1
    let item_obj = gobj::get_obj(item.idx);

    {
        let chara_name = gd.chara.get(cid).get_name().to_owned();
        game_log!("drink-item"; chara=chara_name, item=item.get_name());
    }
    
    let chara = gd.chara.get_mut(cid);

    let eff: i32 = item_obj.eff.into();
    apply_medical_effect(chara, item_obj.medical_effect, eff);
}

/// Eat one item
pub fn eat_item(gd: &mut GameData, il: ItemLocation, cid: CharaId) {
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1
    let item_obj = gobj::get_obj(item.idx);

    {
        let chara_name = gd.chara.get(cid).get_name().to_owned();
        game_log!("eat-item"; chara=chara_name, item=item.get_name());
    }
    
    let chara = gd.chara.get_mut(cid);

    let eff: i32 = item_obj.eff.into();
    apply_medical_effect(chara, item_obj.medical_effect, eff);
}

fn apply_medical_effect(chara: &mut Chara, me: MedicalEffect, eff: i32) {
    match me {
        MedicalEffect::None => (),
        MedicalEffect::Heal => {
            use std::cmp::min;
            chara.hp = min(chara.params.max_hp, chara.hp + eff);
            game_log!("heal-hp"; chara=chara.get_name(), value=eff);
        }
        _ => (),
    }
}

