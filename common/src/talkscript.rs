
use std::collections::HashMap;

/// Hold data of one taliking
#[derive(Serialize, Deserialize)]
pub struct TalkScriptObject {
    pub id: String,
    pub sections: HashMap<String, TalkSection>,
}

impl TalkScriptObject {
    pub fn get_section_text<'a>(&'a self, section: &str) -> Option<&'a str> {
        self.sections[section].text.as_ref().map(|t| t.as_ref())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TalkSection {
    pub text: Option<String>,
    pub action: TalkSectionAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkSectionAction {
    End,
}

