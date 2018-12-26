
use std::slice::{Iter, IterMut};
use crate::objholder::CharaTemplateIdx;

#[derive(Serialize, Deserialize)]
pub struct QuestHolder {
    active_quests: Vec<Quest>,
}

impl QuestHolder {
    pub fn new() -> QuestHolder {
        QuestHolder {
            active_quests: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<Quest> {
        self.active_quests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Quest> {
        self.active_quests.iter_mut()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Quest {
    SlayMonsters {
        idx: CharaTemplateIdx,
        goal: u32,
        killed: u32,
    },
}

