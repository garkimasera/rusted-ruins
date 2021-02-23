use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum ScriptYield {
    Talk { talk: TalkText },
    ShopBuy,
    ShopSell,
    Quest,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TalkText {
    pub text_id: String,
    #[serde(default)]
    pub choices: Vec<(String, String)>,
}
