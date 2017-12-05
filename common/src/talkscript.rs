
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TalkScript {
    pub id: String,
    pub contents: HashMap<String, TalkContent>,
}

#[derive(Deserialize)]
pub struct TalkContent {
    pub text_id: String,
    pub action: TalkAction,
}

#[derive(Deserialize)]
pub enum TalkAction {
    End,
}

