
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
    pub reaction: TalkReaction,
    pub sub_reaction: Option<TalkSubReaction>,
    pub special: SpecialTalkSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkReaction {
    /// End this talk
    End,
    /// Answer from some choices
    Answers {
        answer_texts: Vec<String>,
        dest_sections: Vec<String>,
        /// This answer will be chosen when escape or cancel button is pressed
        esc_answer: Option<u16>,
    },
    /// Jump to another section
    Jump {
        dest_section: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkSubReaction {
}

/// This holds data to represent special talk section.
#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialTalkSection {
    None,
    /// Taught new ruins and dungeons locations by the informant
    InformantRuins,
}

impl Default for SpecialTalkSection {
    fn default() -> SpecialTalkSection {
        SpecialTalkSection::None
    }
}

impl SpecialTalkSection {
    pub fn is_none(&self) -> bool {
        match *self {
            SpecialTalkSection::None => true,
            _ => false,
        }
    }
}

