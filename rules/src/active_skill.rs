use super::Rule;
use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct ActiveSkills(HashMap<ActiveSkillId, ActiveSkill>);

impl Rule for ActiveSkills {
    const NAME: &'static str = "active_skills";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

impl ActiveSkills {
    pub fn get(&self, id: &ActiveSkillId) -> Option<&ActiveSkill> {
        self.0.get(id)
    }
}
