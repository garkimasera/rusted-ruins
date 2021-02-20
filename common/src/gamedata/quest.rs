use super::defs::Reward;
use crate::objholder::CharaTemplateIdx;
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
    pub fn iter(&self) -> Iter<(QuestState, Quest)> {
        self.quests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<(QuestState, Quest)> {
        self.quests.iter_mut()
    }

    pub fn start_new_quest(&mut self, quest: Quest) {
        self.quests.push((QuestState::Active, quest));
    }

    pub fn remove_reward_received(&mut self) {
        self.quests
            .retain(|&(state, _)| state != QuestState::RewardReceived);
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Quest {
    SlayMonsters {
        reward: Reward,
        idx: CharaTemplateIdx,
        goal: u32,
        killed: u32,
    },
}

impl Quest {
    pub fn reward(&self) -> &Reward {
        match self {
            Quest::SlayMonsters { reward, .. } => reward,
        }
    }
}
