
use std::collections::HashMap;

/// Hold data of one taliking
#[derive(Serialize, Deserialize)]
pub struct TalkScriptObject {
    pub id: String,
    pub contents: HashMap<String, TalkContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TalkContent {
    pub text: String,
    pub action: TalkAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkAction {
    End,
}

