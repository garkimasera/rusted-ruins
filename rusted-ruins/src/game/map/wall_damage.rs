use crate::game::Game;
use geom::*;

pub fn wall_damage(game: &mut Game, pos: Vec2d, power: f32) {
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
        map.erase_wall(pos);
    } else {
        tile.wall_hp = wall_hp - damage;
    }
}
