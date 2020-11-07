use crate::game::effect::{do_effect, EffectTarget};
use crate::game::Game;
use common::gamedata::*;
use common::gobj;

pub fn use_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let item = game.gd.get_item(il);
    let item_obj = gobj::get_obj(item.0.idx);

    let use_effect = item_obj.use_effect.as_ref().unwrap();

    match &use_effect.kind[0] {
        EffectKind::SkillLearning { skills } => {
            let mut effect = use_effect.clone();
            let mut skills = skills.clone();
            for attr in &item.0.attributes {
                match attr {
                    ItemAttribute::SkillLearning(skill_kind) => {
                        skills.push(*skill_kind);
                    }
                    _ => (),
                }
            }
            effect.kind[0] = EffectKind::SkillLearning { skills };
            do_effect(game, &effect, Some(cid), EffectTarget::Chara(cid), 1.0, 1.0);
            game.gd.remove_item(il, 1);
            return;
        }
        _ => (),
    }

    do_effect(game, use_effect, Some(cid), EffectTarget::None, 1.0, 1.0);

    game.gd.remove_item(il, 1);
    /*match item_obj.use_effect {
        UseEffect::None => panic!("use invalid item"),
        UseEffect::Deed => {
            assert_eq!(cid, CharaId::Player);


        }
        UseEffect::SkillLearning => {
            for attr in &item.0.attributes {
                match attr {
                    ItemAttribute::SkillLearning(skill_kind) => {
                        let skill_kind = *skill_kind;
                        let chara = gd.chara.get_mut(cid);
                        if chara.skills.learn_new_skill(skill_kind) {
                            game_log_i!("skill-learned"; chara=chara, skill=skill_kind);
                            gd.remove_item(il, 1);
                        } else {
                            game_log_i!("skill-already-learned"; chara=chara, skill=skill_kind);
                        }
                        return;
                    }
                    _ => (),
                }
            }
        }
    }*/
}
