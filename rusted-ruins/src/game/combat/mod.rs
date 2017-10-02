
use super::Game;
use super::chara::*;
use super::animation::*;
use common::objholder::AnimImgIdx;
use common::gobj;
use rand::{thread_rng, Rng};

pub fn attack_neighbor(game: &mut Game, chara_a: CharaId, chara_b: CharaId) {

    let d = {
        let attacker = game.chara_holder.get(chara_a);
        let target = game.chara_holder.get(chara_b);
        let mut attack_power = attacker.params.str as f64;
        let defence_power = target.params.vit as f64 / 2.0;
        attack_power *= thread_rng().gen_range(0.5, 1.0);

        let damage = attack_power - defence_power;
        if damage < 0.0 { 0 }else{ damage as i32 }
    };

    damage(game, chara_a, chara_b, d);
    
    game.anim_queue.push_back(Animation::img_onetile(
        AnimImgIdx(0), game.current_map.chara_pos(chara_b).unwrap()));
}

pub fn damage(game: &mut Game, attacker: CharaId, target: CharaId, damage: i32) {
    let attacker_name = game.chara_holder.get(attacker).name.clone();;
    let mut target = game.chara_holder.get_mut(target);
    
    target.params.hp -= damage;
    game_log!("attack"; attacker=attacker_name, target=target.name);
}

