use crate::game::extrait::*;
use crate::game::Game;
use common::gobj;
use common::objholder::ItemIdx;
use geom::*;

pub fn wall_damage(game: &mut Game<'_>, pos: Vec2d, power: f32) {
    let map = game.gd.get_current_map_mut();
    let tile = &mut map.tile[pos];

    if tile.wall.is_empty() {
        return;
    }

    let wall_hp = tile.wall_hp;

    if wall_hp == std::u16::MAX {
        return;
    }

    let damage = power as u16;

    if wall_hp <= damage {
        let wall_obj = gobj::get_obj(tile.wall.idx().unwrap());
        map.erase_wall(pos);
        for (mining_reward, n) in &wall_obj.mining_rewards {
            if let Some(item_idx) = gobj::id_to_idx_checked::<ItemIdx>(mining_reward) {
                let item = crate::game::item::gen::gen_item_from_idx(item_idx, 1);
                game.gd.add_item_on_tile(pos, item, *n);
            } else {
                warn!("unknown item id for mining reward: \"{}\"", mining_reward);
            }
        }
    } else {
        tile.wall_hp = wall_hp - damage;
    }
}
