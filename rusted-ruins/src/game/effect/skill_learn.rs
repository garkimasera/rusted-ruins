use crate::game::extrait::*;
use common::gamedata::*;

pub fn skill_learn(gd: &mut GameData, cid: CharaId, skills: &[SkillKind]) {
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
