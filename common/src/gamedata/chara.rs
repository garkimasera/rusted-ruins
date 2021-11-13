use super::defs::{AbilityId, AbilityOrigin, Recipe};
use super::faction::FactionId;
use super::item::{EquipItemList, EquipSlotKind, Item, ItemList, ItemLocation, MaterialName};
use super::map::MapId;
use super::modifier::CharaTotalModifier;
use super::site::SiteId;
use super::skill::{CreationKind, SkillKind, SkillList};
use super::{traits::*, unknown_id_err, UniqueId};
use crate::basic::{ArrayStringId, BonusLevel};
use crate::objholder::{CharaTemplateIdx, ItemIdx};
use arrayvec::ArrayString;
use geom::Vec2d;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CharaTemplateObject {
    pub id: String,
    pub img: crate::obj::Img,
    /// Character's race
    pub race: String,
    /// Character's class
    pub class: CharaClass,
    /// Default faction
    pub faction: FactionId,
    /// The frequency of character generation for random map
    pub gen_weight: f32,
    /// Generation level
    /// If it is higher, and the character will be generated on deeper floors
    pub gen_level: u32,
    /// Default AI kind for this character
    pub default_ai_kind: NpcAiKind,
    pub base_attr: CharaBaseAttr,
    pub skill_bonus: HashMap<SkillKind, BonusLevel>,
    /// Learned active skills.
    pub abilities: Vec<AbilityId>,
    pub equips: Vec<(EquipSlotKind, String, u32)>,
}

/// Character classes
#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharaClass(ArrayStringId);

impl CharaClass {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Relationship between one chara to another.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Relationship {
    Ally = 0,
    Friendly,
    Neutral,
    Hostile,
}

/// All data for one character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chara {
    pub idx: CharaTemplateIdx,
    pub name: Option<String>,
    pub attr: CharaAttributes,
    pub class: CharaClass,
    pub faction: FactionId,
    pub lv: u32,
    pub item_list: ItemList,
    pub equip: EquipItemList,
    pub wait_time: u32,
    pub ai: CharaAi,
    pub hp: i32,
    pub sp: f32,
    pub tm: Box<CharaTotalModifier>,
    pub morale: Morale,
    pub traits: Vec<(CharaTraitOrigin, CharaTrait)>,
    pub status: Vec<CharaStatus>,
    pub skills: SkillList,
    pub abilities: Vec<(AbilityOrigin, AbilityId)>,
    /// When talked, execute this script
    pub trigger_talk: Option<String>,
}

/// Character attributes
/// These values are calculated from base params and other factors
/// They are updated by some actions
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CharaAttributes {
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CharaBaseAttr {
    pub base_hp: i32,
    pub str: i16,
    pub vit: i16,
    pub dex: i16,
    pub int: i16,
    pub wil: i16,
    pub cha: i16,
    pub spd: i16,
    /// Carrying power in region map
    pub carry: i16,
    /// Speed in region map
    pub travel_speed: i16,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CharaAttrDiff {
    pub base_hp: i32,
    pub str: i16,
    pub vit: i16,
    pub dex: i16,
    pub int: i16,
    pub wil: i16,
    pub cha: i16,
    pub spd: i16,
}

/// Represents chara status
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CharaStatus {
    /// Sp status
    Hungry,
    /// Sp status
    Weak,
    /// Sp status
    Starving,
    /// Encumbrance status
    Burdened,
    /// Encumbrance status
    Stressed,
    /// Encumbrance status
    Strained,
    /// Encumbrance status
    Overloaded,
    /// Scanned and can open StatusWindow
    Scanned,
    Asleep {
        turn_left: u16,
    },
    Poisoned,
    Work {
        turn_left: u16,
        needed_turn: u16,
        work: Work,
    },
}

impl CharaStatus {
    pub fn turn_left(&self) -> Option<u16> {
        match self {
            &CharaStatus::Asleep { turn_left } | &CharaStatus::Work { turn_left, .. } => {
                Some(turn_left)
            }
            _ => None,
        }
    }

    pub fn turn_left_mut(&mut self) -> Option<&mut u16> {
        match self {
            CharaStatus::Asleep { turn_left } | CharaStatus::Work { turn_left, .. } => {
                Some(turn_left)
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Work {
    Creation {
        kind: CreationKind,
        recipe: Recipe,
        ingredients: Vec<(Item, u32)>,
        material: Option<MaterialName>,
    },
    Harvest {
        item_idx: ItemIdx,
        il: ItemLocation,
    },
}

impl Default for Chara {
    fn default() -> Chara {
        Chara {
            name: None,
            attr: CharaAttributes::default(),
            idx: CharaTemplateIdx::default(),
            class: CharaClass::default(),
            faction: FactionId::default(),
            lv: 0,
            item_list: ItemList::default(),
            equip: EquipItemList::new(&[]),
            wait_time: crate::basic::WAIT_TIME_NUMERATOR,
            ai: CharaAi::default(),
            hp: 100,
            sp: 0.0,
            morale: Morale::default(),
            tm: Box::default(),
            traits: Vec::new(),
            status: Vec::new(),
            skills: SkillList::default(),
            abilities: Vec::new(),
            trigger_talk: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CharaId {
    /// Player is unique character in the game
    Player,
    /// Player ally npcs
    Ally { id: UniqueId },
    /// Npcs used globally in the game
    Global { id: UniqueId },
    /// Unique named npc
    Unique { id: ArrayStringId },
    /// Indexed for a site. This character is associated one site.
    /// Citizens on a town use this id.
    OnSite { sid: SiteId, id: u32 },
    /// Indexed for a map. This character don't appear on other maps.
    /// Randomly generated characters use this id.
    OnMap { mid: MapId, n: u32 },
}

/// Data to determine NPC character's actions
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CharaAi {
    /// AI kind id.
    pub kind: NpcAiKind,
    /// Used for some kind AI that try to return to the initial position.
    pub initial_pos: Vec2d,
    /// Current NPC AI State.
    pub state: AiState,
}

/// Rough kind of NPC AI
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NpcAiKind(ArrayStringId);

impl Default for NpcAiKind {
    fn default() -> NpcAiKind {
        NpcAiKind(ArrayStringId::from("default").unwrap())
    }
}

impl Default for CharaAi {
    fn default() -> CharaAi {
        CharaAi {
            kind: NpcAiKind::default(),
            initial_pos: Vec2d::new(0, 0),
            state: AiState::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum AiState {
    Normal,
    Combat {
        /// Target chara when combat.
        target: CharaId,
    },
    Search {
        turn_count: u32,
    },
}

impl Default for AiState {
    fn default() -> Self {
        AiState::Normal
    }
}

impl AiState {
    pub fn default_search() -> Self {
        AiState::Search { turn_count: 0 }
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, AiState::Normal)
    }

    pub fn is_combat(&self) -> bool {
        matches!(self, AiState::Combat { .. })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharaHolder {
    c: HashMap<CharaId, Chara>,
    on_map: HashMap<CharaId, Chara>,
}

impl CharaHolder {
    pub(crate) fn new() -> CharaHolder {
        CharaHolder {
            c: HashMap::new(),
            on_map: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, cid: CharaId, chara: Chara) {
        match cid {
            CharaId::OnMap { .. } => &mut self.on_map,
            _ => &mut self.c,
        }
        .insert(cid, chara);
    }

    pub fn get(&self, cid: CharaId) -> &Chara {
        match cid {
            CharaId::OnMap { .. } => &self.on_map,
            _ => &self.c,
        }
        .get(&cid)
        .unwrap_or_else(|| unknown_id_err(cid))
    }

    pub fn get_mut(&mut self, cid: CharaId) -> &mut Chara {
        match cid {
            CharaId::OnMap { .. } => &mut self.on_map,
            _ => &mut self.c,
        }
        .get_mut(&cid)
        .unwrap_or_else(|| unknown_id_err(cid))
    }

    pub fn exist(&self, cid: CharaId) -> bool {
        match cid {
            CharaId::OnMap { .. } => &self.on_map,
            _ => &self.c,
        }
        .get(&cid)
        .is_some()
    }

    pub(crate) fn remove_chara(&mut self, cid: CharaId) {
        match cid {
            CharaId::OnMap { .. } => &mut self.on_map,
            _ => &mut self.c,
        }
        .remove(&cid);
    }

    pub(crate) fn replace_on_map_chara(
        &mut self,
        next: HashMap<CharaId, Chara>,
    ) -> HashMap<CharaId, Chara> {
        std::mem::replace(&mut self.on_map, next)
    }

    pub(crate) fn retain<F: FnMut(&CharaId, &Chara) -> bool>(&mut self, mut f: F) {
        self.c.retain(|cid, chara| f(cid, chara));
        self.on_map.retain(|cid, chara| f(cid, chara));
    }
}

/// When a chara is talked to, talk will be start from the section of specified TalkScript
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CharaTalk {
    /// Id of TalkScriptObject
    pub id: String,
    /// Section of the TalkScript
    pub section: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Morale(i8);
