mod attack;
mod misc;
mod range;
mod restore;
pub mod skill_learn;

pub use attack::*;
pub use range::*;

use crate::game::extrait::*;
use crate::game::target::Target;
use crate::game::{Animation, Game, InfoGetter};
use common::gamedata::*;
use common::gobj;
use common::objholder::TileIdx;
use geom::*;
use ordered_float::NotNan;

pub fn do_effect<T: Into<Target>>(
    game: &mut Game,
    effect: &Effect,
    cause: Option<CharaId>,
    target: T,
    power: f32,
    hit: f32,
) {
    let target = target.into();
    // Target tiles
    let tiles = get_tiles(game, effect, target, cause);
    // Target characters
    let cids = get_cids(game, effect, &tiles);

    let power = effect.base_power.calc(power);
    let hit = f32::from(effect.hit) + hit;

    for effect_kind in &effect.kind {
        match effect_kind {
            EffectKind::RestoreHp => {
                for cid in &cids {
                    self::restore::restore_hp(game, *cid, power);
                }
            }
            EffectKind::Melee { element } => {
                for cid in &cids {
                    self::attack::melee_attack(game, cause.unwrap(), *cid, power, hit, *element);
                }
            }
            EffectKind::Ranged { element } => {
                for cid in &cids {
                    self::attack::ranged_attack(game, cause.unwrap(), *cid, power, hit, *element);
                }
            }
            EffectKind::Explosion { element } => {
                for cid in &cids {
                    self::attack::explosion_attack(
                        game,
                        cause.unwrap(),
                        *cid,
                        power,
                        hit,
                        *element,
                    );
                }
            }
            EffectKind::Status { status } => {
                for cid in &cids {
                    cause_status(game, *cid, power, *status);
                }
            }
            EffectKind::WallDamage => {
                for pos in &tiles {
                    crate::game::map::wall_damage::wall_damage(game, *pos, power);
                }
            }
            EffectKind::SkillLearning { skills } => {
                for cid in &cids {
                    self::skill_learn::skill_learn(&mut game.gd, *cid, skills);
                }
            }
            EffectKind::PlaceTile { tile } => {
                if let Some(tile_idx) = gobj::id_to_idx_checked::<TileIdx>(tile) {
                    let map = game.gd.get_current_map_mut();

                    for &pos in &tiles {
                        map.set_tile(pos, tile_idx, None);
                    }
                }
            }
            EffectKind::GenItem { id } => {
                for pos in &tiles {
                    self::misc::gen_item(game, id, *pos);
                }
            }
            other => {
                error!("unimplemented effect: {:?}", other);
            }
        }
    }
    // Animation
    match effect.anim_kind {
        EffectAnimKind::None => (),
        EffectAnimKind::Chara => {
            let tiles = cids
                .iter()
                .map(|cid| game.gd.chara_pos(*cid).unwrap())
                .collect();
            if !effect.anim_img.is_empty() {
                let idx = gobj::id_to_idx(&effect.anim_img);
                let anim = Animation::img_tiles(idx, tiles);
                game.anim_queue.push(anim);
            }
        }
        EffectAnimKind::Tile => {
            if !effect.anim_img.is_empty() {
                let idx = gobj::id_to_idx(&effect.anim_img);
                let anim = Animation::img_tiles(idx, tiles);
                game.anim_queue.push(anim);
            }
        }
        EffectAnimKind::Shot => {
            if !effect.anim_img_shot.is_empty() {
                let start = game
                    .gd
                    .chara_pos(cause.unwrap())
                    .expect("chara position search error");
                let idx = gobj::id_to_idx(&effect.anim_img_shot);
                game.anim_queue.push(Animation::shot(idx, start, tiles[0]));
            }
        }
    }
    // Sound
    if !effect.sound.is_empty() {
        audio::play_sound(&effect.sound);
    }
}

// Get characters list in range of the effect.
fn get_cids(game: &Game, _effect: &Effect, tiles: &[Coords]) -> Vec<CharaId> {
    let map = game.gd.get_current_map();
    let mut cids = vec![];

    for pos in tiles {
        if let Some(chara) = map.tile[*pos].chara {
            cids.push(chara)
        }
    }
    cids
}

// Get tile positions of the effect
fn get_tiles(game: &Game, effect: &Effect, target: Target, cause: Option<CharaId>) -> Vec<Coords> {
    let cause = cause.and_then(|cause| game.gd.chara_pos(cause));
    let target = match target {
        Target::None => {
            return vec![];
        }
        Target::Tile(pos) => pos,
        Target::Chara(cid) => {
            if let Some(pos) = game.gd.chara_pos(cid) {
                pos
            } else {
                return vec![];
            }
        }
    };
    let map = game.gd.get_current_map();
    if let Some(shape) = to_shape(effect, target, cause) {
        shape
            .iter()
            .into_iter()
            .filter(|pos| map.is_inside(*pos))
            .collect()
    } else {
        vec![]
    }
}

fn to_shape(effect: &Effect, target: Coords, _cause: Option<Coords>) -> Option<Shape> {
    match effect.shape {
        ShapeKind::OneTile => Some(Shape::OneTile { pos: target }),
        ShapeKind::Line => unimplemented!(),
        ShapeKind::Circle => Some(Shape::Circle {
            center: target,
            radius: effect.size,
        }),
    }
}

// Cause status effect to given chara.
fn cause_status(game: &mut Game, cid: CharaId, power: f32, status: StatusEffect) {
    let chara = game.gd.chara.get_mut(cid);

    match status {
        StatusEffect::Asleep => {
            chara.add_status(CharaStatus::Asleep {
                turn_left: power as u16,
            });
            game_log_i!("fall-asleep"; chara=chara);
        }
        StatusEffect::Poison => {
            chara.add_status(CharaStatus::Poisoned);
            game_log_i!("poisoned"; chara=chara);
        }
        StatusEffect::Scanned => {
            chara.add_status(CharaStatus::Scanned);
            game_log_i!("scanned"; chara=chara);
        }
    }
}

#[extend::ext(pub)]
impl BasePower {
    fn calc(&self, factor: f32) -> f32 {
        let factor = NotNan::new(factor).unwrap();
        let base_power = self.0 * factor;
        let power_var = self.1 * factor;
        let power_min = std::cmp::max(base_power - power_var, NotNan::new(0.0).unwrap());
        let power_max = base_power + power_var;
        let power = if power_max > power_min {
            rng::gen_range(power_min..power_max)
        } else {
            power_min
        };
        power.into_inner()
    }

    fn calc_without_var(&self, factor: f32) -> f32 {
        self.0.into_inner() * factor
    }
}
