use common::gamedata::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct ActiveSkills(HashMap<String, ActiveSkill>);

impl ActiveSkills {
    fn join(&mut self, other: ActiveSkills) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }

    pub fn get(&self, id: &str) -> Option<&ActiveSkill> {
        self.get_opt(&Some(id))
    }

    pub fn get_opt<S: AsRef<str>>(&self, id: &Option<S>) -> Option<&ActiveSkill> {
        if let Some(id) = id {
            self.0.get(id.as_ref())
        } else {
            None
        }
    }

    pub fn join_from_dir(&mut self, dir: &Path) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }
            let path = entry.path();
            let extension = path.extension();
            if extension.is_none() {
                continue;
            }
            let extension = extension.unwrap();
            if extension != "ron" {
                continue;
            }

            let active_skills = super::read_from_file(path);

            self.join(active_skills);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSkill {
    pub effect: Effect,
}
