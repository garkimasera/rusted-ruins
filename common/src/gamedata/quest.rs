use super::{defs::Reward, SiteId, Time};
use crate::hashmap::HashSet;
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
    pub custom_quests: Vec<CustomQuest>,
    pub completed_custom_quests: HashSet<String>,
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
    ItemDelivering { items: Vec<(ItemIdx, u32)> },
    DestroyBase,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CustomQuest {
    pub id: String,
    pub phase: String,
}
