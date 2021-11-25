use super::{defs::Reward, SiteId, Time};
use crate::objholder::ItemIdx;
use std::slice::{Iter, IterMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TownQuestState {
    Active,
    Reportable,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QuestHolder {
    pub town_quests: Vec<(TownQuestState, TownQuest)>,
    // custom_quests: Vec<_>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TownQuest {
    pub sid: SiteId,
    pub text_id: String,
    pub deadline: Option<u32>,
    pub reward: Reward,
    pub kind: TownQuestKind,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TownQuestKind {
    ItemDelivering { idx: ItemIdx, n: u32 },
}
