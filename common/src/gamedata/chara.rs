
use std::collections::HashMap;
use objholder::CharaTemplateIdx;
use super::item::{ItemList, EquipItemList};
use super::map::MapId;

/// Character's races
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum Race {
    Animal, Devil, Human, Phantom, Slime,
}

/// Relationship between one chara to another.
///         |A|F|N|H
/// ALLY    |A|F|N|H
/// FRIENDLY|F|F|N|H
/// NEUTRAL |N|N|N|N
/// HOSTILE |H|H|N|F
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u32)]
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
    pub name: String,
    pub params: CharaParams,
    pub base_params: CharaBaseParams,
    pub template: CharaTemplateIdx,
    pub item_list: ItemList,
    pub equip: EquipItemList,
    pub wait_time: f32,
    pub hp: i32,
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
    pub str: u32,
    /// Vitality
    pub vit: u32,
    /// Dexterity
    pub dex: u32,
    /// Intelligence
    pub int: u32,
    /// Will
    pub wil: u32,
    /// Charisma
    pub cha: u32,
    /// Speed
    pub spd: u32,
}

/// Character base parameters
#[derive(Serialize, Deserialize, Default)]
pub struct CharaBaseParams {
    /// Character level
    pub level: u32,
    pub max_hp: i32,
    pub str: u32,
    pub vit: u32,
    pub dex: u32,
    pub int: u32,
    pub wil: u32,
    pub cha: u32,
    pub spd: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u32)]
enum CharaState {
    /// This character was removed from game.
    Disable,
    /// Its HP is under 0.
    /// This character will be drawed, but the state will becomes Disable next turn.
    Dying,
    Normal,
}

impl Default for Chara {
    fn default() -> Chara {
        Chara {
            name: "Unknown".to_owned(),
            params: CharaParams::default(),
            base_params: CharaBaseParams::default(),
            template: CharaTemplateIdx(0),
            item_list: ItemList::for_chara(),
            equip: EquipItemList::new(&[]),
            wait_time: 100.0,
            hp: 100,
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

#[derive(Serialize, Deserialize)]
pub struct CharaHolder(pub(crate) HashMap<CharaId, Chara>);

impl CharaHolder {
    pub(crate) fn new() -> CharaHolder {
        CharaHolder(HashMap::new())
    }
    
    pub fn get(&self, cid: CharaId) -> &Chara {
        self.0.get(&cid).expect(&super::unknown_id_err(cid))
    }

    pub fn get_mut(&mut self, cid: CharaId) -> &mut Chara {
        self.0.get_mut(&cid).expect(&super::unknown_id_err(cid))
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
    Event(::event::EventTrigger),
}

/// When a chara is talked to, talk will be start from the section of specified TalkScript
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CharaTalk {
    /// Id of TalkScriptObject
    pub id: String,
    /// Section of the TalkScript
    pub section: String,
    /// If the talk include custom event, for example shopping
    /// This data will be used to specify that event
    pub event_data: Option<String>,
}

