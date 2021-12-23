use crate::game::extrait::*;
use crate::game::Game;
use common::gamedata::*;

pub fn skill_learn(game: &mut Game, cid: CharaId, skills: &[SkillKind]) {
    let gd = &mut game.gd;

    for skill_kind in skills {
        let skill_kind = *skill_kind;
        let chara = gd.chara.get_mut(cid);
        if chara.skills.learn_new_skill(skill_kind) {
            game_log!("skill-learned"; chara=chara, skill=skill_kind);
        } else {
            game_log!("skill-already-learned"; chara=chara, skill=skill_kind);
        }
    }
}
