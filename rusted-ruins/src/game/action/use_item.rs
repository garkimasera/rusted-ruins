use crate::game::effect::do_effect;
use crate::game::target::Target;
use crate::game::Game;
use common::gamedata::*;
use common::gobj;

pub fn use_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let item = game.gd.get_item(il);
    let item_obj = gobj::get_obj(item.0.idx);

    let use_effect = item_obj.use_effect.as_ref().unwrap();

    match use_effect {
        UseEffect::Effect(effect) => {
            /* let mut effect = effect.clone();
            for effect_kind in &mut effect.kind {
                match effect_kind {
                    EffectKind::SkillLearning { skills } => {
                        for attr in &item.0.attributes {
                            match attr {
                                ItemAttribute::SkillLearning(skill_kind) => {
                                    skills.push(*skill_kind);
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }*/
            do_effect(game, &effect, Some(cid), Target::None, 1.0, 1.0);
        }
        UseEffect::Deed => {
            todo!();
        }
    }

    game.gd.remove_item(il, 1);
}
