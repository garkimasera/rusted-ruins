use super::defs::Reward;
use crate::objholder::ItemIdx;
use std::slice::{Iter, IterMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum QuestState {
    Active,
    Completed,
    RewardReceived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestHolder {
    quests: Vec<(QuestState, Quest)>,
}

impl Default for QuestHolder {
    fn default() -> Self {
        QuestHolder { quests: Vec::new() }
    }
}

impl QuestHolder {
    pub fn iter(&self) -> Iter<'_, (QuestState, Quest)> {
        self.quests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (QuestState, Quest)> {
        self.quests.iter_mut()
    }

    pub fn start_new_quest(&mut self, quest: Quest) {
        self.quests.push((QuestState::Active, quest));
    }

    pub fn remove(&mut self, i: usize) {
        self.quests.remove(i);
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Quest {
    ItemDelivering {
        reward: Reward,
        idx: ItemIdx,
        n: u32,
    },
}

impl Quest {
    pub fn reward(&self) -> &Reward {
        match self {
            Quest::ItemDelivering { reward, .. } => reward,
        }
    }
}
