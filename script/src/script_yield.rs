use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum ScriptYield {
    Talk { talk: TalkText },
    ShopBuy,
    ShopSell,
    QuestOffer,
    QuestReport,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TalkText {
    pub text_id: String,
    #[serde(default)]
    pub choices: Vec<String>,
    #[serde(default)]
    pub target_chara: Option<String>,
}
