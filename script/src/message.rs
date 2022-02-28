use common::gamedata::{GameData, Value};

pub(crate) enum ScriptMessage {
    Finish,
    Fail,
    UiRequest(UiRequest),
    Exec(Box<dyn FnOnce(&mut GameData) -> Value + Send + 'static>),
    Method(GameMethod),
}

pub enum ScriptResult {
    Finish,
    UiRequest(UiRequest),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UiRequest {
    Talk { talk: TalkText },
    ShopBuy,
    ShopSell,
    QuestOffer,
    QuestReport,
    InstallAbilitySlot,
    InstallExtendSlot,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TalkText {
    pub text_id: String,
    pub choices: Vec<String>,
    pub target_chara: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GameMethod {
    CompleteCustomQuest { id: String },
    CustomQuestStarted { id: String },
    GenDungeons,
    GenPartyChara { id: String, lv: u32 },
    HasEmptyForParty,
    NumberOfItem { id: String },
    ReceiveItem { id: String, n: u32 },
    ReceiveMoney { amount: i64 },
    RemoveItem { id: String, n: u32 },
    ResurrectPartyMembers,
    StartCustomQuest { id: String, phase: String },
    LearnSkill { skill: String },
}
