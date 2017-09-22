
use super::Game;
use super::Animation;
use super::CharaId;
use super::combat;
use array2d::*;
use common::chara::*;

pub fn try_move(game: &mut Game, chara_id: CharaId, dir: Direction) -> bool {
    if dir.as_vec() == (0, 0) { return true; } // Move to current tile always success
    let dest_tile = game.current_map.chara_pos(chara_id).unwrap() + dir.as_vec();
    let push_move_anim = |game: &mut Game| {
        if chara_id == CharaId::Player {
            game.anim_queue.push_back(Animation::player_move(dir));
        }
    };

    if !game.current_map.is_movable(dest_tile) {
        return false;
    }

    let other_chara = game.current_map.get_chara(dest_tile);
    if other_chara.is_none() {
        game.current_map.move_chara(chara_id, dir);
        push_move_anim(game);
    }else{
        let other_chara = other_chara.unwrap();
        let rel = game.chara_holder.relative_relation(chara_id, other_chara);
        match rel {
            Relationship::ALLY | Relationship::FRIENDLY |Relationship::NEUTRAL => {
                game.current_map.move_chara(chara_id, dir);
                push_move_anim(game);
            },
            Relationship::HOSTILE => {
                combat::attack_neighbor(game, chara_id, other_chara);
            },
        }
    }
    true
}




