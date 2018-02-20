
use rng;
use super::Game;
use super::animation::*;
use common::gobj;
use common::objholder::AnimImgIdx;
use common::gamedata::chara::{CharaId, Chara};
use common::gamedata::item::*;

pub enum DamageKind {
    CloseRangeAttack,
}

pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    // Damage calculation
    let damage = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        let defence_power = target.params.vit as i32;
        let weapon_data = get_weapon_data(attacker);
        let weapon_dice_result = rng::dice(weapon_data.dice_n, weapon_data.dice_x);

        let damage_coef = 256 + attacker.params.str as i32 * 16;

        let damage = weapon_dice_result.saturating_mul(damage_coef) / 256;
        
        if damage < 0 { 0 }else{ damage as i32 }
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("attack"; attacker=attacker.name, target=target.name, damage=damage);
    }
    // Damage processing
    super::chara::damage(game, target, damage, DamageKind::CloseRangeAttack);
    // Animation pushing
    game.anim_queue.push_back(Animation::img_onetile(
        AnimImgIdx(0), game.gd.get_current_map().chara_pos(target).unwrap()));
}

struct WeaponData {
    dice_n: i32,
    dice_x: i32,
}

fn get_weapon_data(chara: &Chara) -> WeaponData {
    if let Some(weapon) = chara.equip.item(ItemKind::Weapon, 0) {
        let item_obj = gobj::get_obj(weapon.idx);
        match item_obj.content {
            ItemContent::Weapon { dice_n, dice_x, kind } => {
                WeaponData {
                    dice_n, dice_x
                }
            }
            _ => unreachable!()
        }
    } else {
        WeaponData {
            dice_n: 4, dice_x: 4
        }
    }
}
