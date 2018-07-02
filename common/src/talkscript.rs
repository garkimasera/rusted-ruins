
use hashmap::HashMap;
use std::borrow::Cow;
use gamedata::event::EventTrigger;

/// Hold data of one taliking
#[derive(Serialize, Deserialize)]
pub struct TalkScriptObject {
    pub id: String,
    pub sections: HashMap<String, TalkSection>,
}

impl TalkScriptObject {
    /// Get text id of given section
    pub fn get_section_text<'a>(&'a self, section: &str) -> Option<Cow<'a, str>> {
        match self.sections[section] {
            TalkSection::Normal { ref text, .. } =>  {
                let s = if let Some(ref text) = *text {
                    Cow::Borrowed(text.as_ref())
                } else {
                    Cow::Owned(format!("{}-{}", self.id, section))
                };
                Some(s)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkSection {
    Normal {
        text: Option<String>,
        answers: Vec<String>,
        dest_sections: Vec<String>,
        default_dest_section: Option<String>,
    },
    Reaction {
        reaction: TalkReaction,
        next_section: String,
    },
    Special {
        special: SpecialTalkSection,
        dest_sections: Vec<String>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum TalkSectionKind {
    Normal, Reaction, Special,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
#[serde(tag = "kind")]
pub enum TalkReaction {
    EventTrigger {
        trigger: EventTrigger
    },
}

/// This holds data to represent special talk section.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SpecialTalkSection {
    /// Taught new ruins and dungeons locations by the informant
    InformantDungeons,
    /// Open shop window (buy)
    ShopBuy,
    /// Open shop window (sell)
    ShopSell,
}

