
use common::basic::SKILL_EXP_LVUP;
use common::gamedata::skill::*;

pub trait SkillListEx {
    fn add_exp(&mut self, kind: SkillKind, value: u32);
}

impl SkillListEx for SkillList {
    fn add_exp(&mut self, kind: SkillKind, add_exp: u32) {
        if let Some(ref mut exp) = self.exp {
            if let Some(skill_exp) = exp.get_mut(&kind) {
                let add_exp = if add_exp > SKILL_EXP_LVUP as u32 {
                    SKILL_EXP_LVUP as u32
                } else {
                    add_exp
                };
                let sum = *skill_exp as u32 + add_exp;
                *skill_exp = if sum >= SKILL_EXP_LVUP.into() { // Level up
                    if let Some(skill_level) = self.skills.get_mut(&kind) {
                        *skill_level += 1;
                    }
                    (sum - SKILL_EXP_LVUP as u32) as u16
                } else {
                    sum as u16
                };
            }
        }
    }
}

