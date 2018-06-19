
use std::collections::HashMap;
use objholder::CharaTemplateIdx;
use super::item::{ItemList, EquipItemList};
use super::map::MapId;
use super::skill::SkillList;
use super::event::EventTrigger;
use super::unknown_id_err;

/// Character's races
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Race {
    Animal, Devil, Human, Bug, Phantom, Slime,
}

/// Character classes
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum CharaClass {
    // Playable classes
    Archeologist, Rogue,
    // Npc classes
    Civilian,
}

impl Default for CharaClass {
    fn default() -> CharaClass {
        CharaClass::Civilian
    }
}

/// Relationship between one chara to another.
///         |A|F|N|H
/// ALLY    |A|F|N|H
/// FRIENDLY|F|F|N|H
/// NEUTRAL |N|N|N|N
/// HOSTILE |H|H|N|F
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Relationship {
    ALLY = 0,
    FRIENDLY,
    NEUTRAL,
    HOSTILE,
}

impl Relationship {
    pub fn relative(&self, other: Relationship) -> Relationship {
        use self::Relationship::*;
        match (*self, other) {
            (ALLY, o) => o,
            (FRIENDLY, ALLY) => FRIENDLY, (FRIENDLY, FRIENDLY) => FRIENDLY,
            (FRIENDLY, NEUTRAL) => NEUTRAL, (FRIENDLY, HOSTILE) => HOSTILE,
            (NEUTRAL, _) => NEUTRAL,
            (HOSTILE, ALLY) => HOSTILE, (HOSTILE, FRIENDLY) => HOSTILE,
            (HOSTILE, NEUTRAL) => NEUTRAL, (HOSTILE, HOSTILE) => FRIENDLY,
        }
    }
}

/// All data for one character
#[derive(Serialize, Deserialize)]
pub struct Chara {
    pub name: Option<String>,
    pub params: CharaParams,
    pub base_params: CharaBaseParams,
    pub template: CharaTemplateIdx,
    pub class: CharaClass,
    pub item_list: ItemList,
    pub equip: EquipItemList,
    pub wait_time: u32,
    pub ai: CharaAI,
    pub hp: i32,
    pub nutrition: i32,
    pub status: Vec<CharaStatus>,
    pub skills: SkillList,
    /// Relationship to player character
    pub rel: Relationship,
    /// Trigger for event
    pub trigger: Option<(CharaTriggerKind, TriggerAction)>,
    /// Talk attribute
    pub talk: Option<CharaTalk>,
}

/// Character parameters
/// These values are calculated from base params and other factors
/// They are updated by some actions
#[derive(Serialize, Deserialize, Default)]
pub struct CharaParams {
    /// Max HP
    pub max_hp: i32,
    /// Strength
    pub str: u16,
    /// Vitality
    pub vit: u16,
    /// Dexterity
    pub dex: u16,
    /// Intelligence
    pub int: u16,
    /// Will
    pub wil: u16,
    /// Charisma
    pub cha: u16,
    /// Speed
    pub spd: u16,
    /// Range of view in tile
    pub view_range: i32,
}

/// Character base parameters
#[derive(Serialize, Deserialize, Default)]
pub struct CharaBaseParams {
    /// Character level
    pub level: u16,
    pub max_hp: i32,
    pub str: u16,
    pub vit: u16,
    pub dex: u16,
    pub int: u16,
    pub wil: u16,
    pub cha: u16,
    pub spd: u16,
}

/// Represents chara status
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub enum CharaStatus {
    Hungry,
    Asleep { turn_left: u16 },
    Poisoned,
}

impl Default for Chara {
    fn default() -> Chara {
        Chara {
            name: None,
            params: CharaParams::default(),
            base_params: CharaBaseParams::default(),
            template: CharaTemplateIdx(0),
            class: CharaClass::default(),
            item_list: ItemList::for_chara(),
            equip: EquipItemList::new(&[]),
            wait_time: ::basic::WAIT_TIME_START,
            ai: CharaAI::default(),
            hp: 100,
            nutrition: 0,
            status: Vec::new(),
            skills: SkillList::default(),
            rel: Relationship::NEUTRAL,
            trigger: None,
            talk: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CharaKind {
    /// Player is unique character in the game
    Player,
    /// Indexed for a map. This character don't appear on other maps
    OnMap,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CharaId {
    /// Player is unique character in the game
    Player,
    /// Indexed for a map. This character don't appear on other maps
    OnMap { mid: MapId, n: u32 },
}

/// Data to determine NPC character's actions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct CharaAI {
    pub kind: NpcAIKind,
}

/// Rough kind of NPC AI
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum NpcAIKind {
    /// This npc does not do anything.
    None,
    /// This npc will not move
    NoMove,
    /// This npc will chase near enemies, and try melee atacks
    Melee,
}

impl Default for CharaAI {
    fn default() -> CharaAI {
        CharaAI {
            kind: NpcAIKind::None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharaHolder(pub(crate) HashMap<CharaId, Chara>);

impl CharaHolder {
    pub(crate) fn new() -> CharaHolder {
        CharaHolder(HashMap::new())
    }
    
    pub fn get(&self, cid: CharaId) -> &Chara {
        self.0.get(&cid).unwrap_or_else(|| unknown_id_err(cid))
    }

    pub fn get_mut(&mut self, cid: CharaId) -> &mut Chara {
        self.0.get_mut(&cid).unwrap_or_else(|| unknown_id_err(cid))
    }

    pub fn iter_charaid(&self) -> ::std::collections::hash_map::Keys<CharaId, Chara> {
        self.0.keys()
    }

    pub(crate) fn remove_chara(&mut self, cid: CharaId) {
        self.0.remove(&cid);
    }
}

/// Represents triggers for one chara
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharaTriggerKind {
    /// When the chara is died
    Die,
}

/// Action after trigger occured
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TriggerAction {
    /// Trigger for event handling
    Event(EventTrigger),
}

/// When a chara is talked to, talk will be start from the section of specified TalkScript
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CharaTalk {
    /// Id of TalkScriptObject
    pub id: String,
    /// Section of the TalkScript
    pub section: String,
}

