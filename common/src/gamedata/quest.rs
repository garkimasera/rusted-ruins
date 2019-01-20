
use std::slice::{Iter, IterMut};
use crate::objholder::CharaTemplateIdx;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum QuestState {
    Active, Completed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestHolder {
    quests: Vec<(QuestState, Quest)>,
}

impl QuestHolder {
    pub fn new() -> QuestHolder {
        QuestHolder {
            quests: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<(QuestState, Quest)> {
        self.quests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<(QuestState, Quest)> {
        self.quests.iter_mut()
    }

    pub fn start_new_quest(&mut self, quest: Quest) {
        self.quests.push((QuestState::Active, quest));
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

