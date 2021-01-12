mod attack;
mod deed;
mod range;
mod skill_learn;

pub use attack::weapon_to_effect;
pub use range::*;

use crate::game::extrait::CharaStatusOperation;
use crate::game::target::Target;
use crate::game::InfoGetter;
use crate::game::{Animation, Game};
use common::gamedata::*;
use common::gobj;
use geom::*;

pub fn do_effect<T: Into<Target>>(
    game: &mut Game,
    effect: &Effect,
    cause: Option<CharaId>,
    target: T,
    power: f32,
    hit_power: f32,
) {
    let target = target.into();
    // Target characters
    let cids = get_cids(game, effect, target);
    // Target tiles
    let tiles = get_tiles(game, effect, target);

    for effect_kind in &effect.kind {
        match effect_kind {
            EffectKind::Melee { element } => {
                for cid in &cids {
                    self::attack::melee_attack(
                        game,
                        cause.unwrap(),
                        *cid,
                        power,
                        hit_power,
                        *element,
                    );
                }
            }
            EffectKind::Ranged { element } => {
                for cid in &cids {
                    self::attack::ranged_attack(
                        game,
                        cause.unwrap(),
                        *cid,
                        power,
                        hit_power,
                        *element,
                    );
                }
            }
            EffectKind::Explosion { element } => {
                for cid in &cids {
                    self::attack::explosion_attack(
                        game,
                        cause.unwrap(),
                        *cid,
                        power,
                        hit_power,
                        *element,
                    );
                }
            }
            EffectKind::Status { status } => {
                let cids = get_cids(game, effect, target);
                for cid in &cids {
                    cause_status(game, *cid, power, *status);
                }
            }
            EffectKind::WallDamage => {
                for pos in &tiles {
                    crate::game::map::wall_damage::wall_damage(game, *pos, power);
                }
            }
            EffectKind::Deed => {
                self::deed::use_deed(game);
            }
            EffectKind::SkillLearning { skills } => {
                let cids = get_cids(game, effect, target);
                for cid in &cids {
                    self::skill_learn::skill_learn(game, *cid, skills);
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
                let anim = Animation::img_tiles(idx, tiles.clone());
                game.anim_queue.push(anim);
            }
        }
        EffectAnimKind::Shot => {
            if !effect.anim_img_shot.is_empty() {
                let start = game
                    .gd
                    .chara_pos(cause.unwrap())
                    .expect("chara position search error");
                let tile = game
                    .gd
                    .chara_pos(cids[0])
                    .expect("chara position search error");
                let idx = gobj::id_to_idx(&effect.anim_img_shot);
                game.anim_queue.push(Animation::shot(idx, start, tile));
            }
        }
    }
    // Sound
    if !effect.sound.is_empty() {
        audio::play_sound(&effect.sound);
    }
}

// Get characters list in range of the effect.
fn get_cids(game: &Game, _effect: &Effect, target: Target) -> Vec<CharaId> {
    // TODO: multiple cids will be needed for widely ranged effect.
    match target {
        Target::None => vec![],
        Target::Tile(pos) => {
            if let Some(cid) = game.gd.get_current_map().get_chara(pos) {
                vec![cid]
            } else {
                vec![]
            }
        }
        Target::Chara(cid) => vec![cid],
    }
}

// Get tile positions of the effect
fn get_tiles(game: &Game, _effect: &Effect, target: Target) -> Vec<Vec2d> {
    let center = match target {
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
    vec![center]
}

// Cause status effect to given chara.
fn cause_status(game: &mut Game, cid: CharaId, power: f32, status: StatusEffect) {
    let chara = game.gd.chara.get_mut(cid);

    match status {
        StatusEffect::Asleep => {
            chara.add_status(CharaStatus::Asleep {
                turn_left: power as u16,
            });
            game_log!("fall-asleep"; chara=chara);
        }
        StatusEffect::Poison => {
            chara.add_status(CharaStatus::Poisoned);
            game_log!("poisoned"; chara=chara);
        }
        StatusEffect::Scanned => {
            chara.add_status(CharaStatus::Scanned);
            game_log!("scanned"; chara=chara);
        }
    }
}
