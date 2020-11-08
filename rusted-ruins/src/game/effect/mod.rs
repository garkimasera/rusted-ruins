mod attack;
mod deed;
mod skill_learn;

pub use attack::weapon_to_effect;

use crate::game::extrait::CharaStatusOperation;
use crate::game::Game;
use crate::game::InfoGetter;
use common::gamedata::*;
use geom::*;

#[derive(Clone, Copy, Debug)]
pub enum EffectTarget {
    None,
    Tile(Vec2d),
    Chara(CharaId),
}

impl From<Vec2d> for EffectTarget {
    fn from(pos: Vec2d) -> EffectTarget {
        EffectTarget::Tile(pos)
    }
}

impl From<CharaId> for EffectTarget {
    fn from(cid: CharaId) -> EffectTarget {
        EffectTarget::Chara(cid)
    }
}

pub fn do_effect<T: Into<EffectTarget>>(
    game: &mut Game,
    effect: &Effect,
    cause: Option<CharaId>,
    target: T,
    power: f32,
    hit_power: f32,
) {
    let target = target.into();

    for effect_kind in &effect.kind {
        match effect_kind {
            EffectKind::Melee { element } => {
                let cids = get_cids(game, effect, target);
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
                let tile = game.gd.chara_pos(cids[0]).unwrap();
                game.anim_queue.push_effect(effect, tile, None);
                if !effect.sound.is_empty() {
                    audio::play_sound(&effect.sound);
                }
                return;
            }
            EffectKind::Ranged { element } => {
                let cids = get_cids(game, effect, target);
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
                let start = cause.map(|cause| {
                    game.gd
                        .chara_pos(cause)
                        .expect("chara position search error")
                });
                let tile = game
                    .gd
                    .chara_pos(cids[0])
                    .expect("chara position search error");
                game.anim_queue.push_effect(effect, tile, start);
                if !effect.sound.is_empty() {
                    audio::play_sound(&effect.sound);
                }
                return;
            }
            EffectKind::Status { status } => {
                let cids = get_cids(game, effect, target);
                for cid in &cids {
                    cause_status(game, *cid, power, *status);
                }
                return;
            }
            EffectKind::WallDamage => {
                let pos_list = get_pos(game, effect, target);
                for pos in &pos_list {
                    crate::game::map::wall_damage::wall_damage(game, *pos, power);
                }
            }
            EffectKind::Deed => {
                self::deed::use_deed(game);
                return;
            }
            EffectKind::SkillLearning { skills } => {
                let cids = get_cids(game, effect, target);
                for cid in &cids {
                    self::skill_learn::skill_learn(game, *cid, skills);
                }
                return;
            }
            other => {
                error!("unimplemented effect: {:?}", other);
            }
        }
    }
}

// Get characters list in range of the effect.
fn get_cids(game: &Game, _effect: &Effect, target: EffectTarget) -> Vec<CharaId> {
    // TODO: multiple cids will be needed for widely ranged effect.
    match target {
        EffectTarget::None => vec![],
        EffectTarget::Tile(pos) => {
            if let Some(cid) = game.gd.get_current_map().get_chara(pos) {
                vec![cid]
            } else {
                vec![]
            }
        }
        EffectTarget::Chara(cid) => vec![cid],
    }
}

// Get tile positions of the effect
fn get_pos(game: &Game, _effect: &Effect, target: EffectTarget) -> Vec<Vec2d> {
    let center = match target {
        EffectTarget::None => {
            return vec![];
        }
        EffectTarget::Tile(pos) => pos,
        EffectTarget::Chara(cid) => {
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
    }
}
