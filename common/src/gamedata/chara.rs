
use std::collections::HashMap;
use objholder::CharaTemplateIdx;
use super::item::Inventory;
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
    pub template: CharaTemplateIdx,
    pub inventory: Inventory,
    pub wait_time: f32,
    pub hp: i32,
    /// Relationship to player character
    pub rel: Relationship,
}

#[derive(Serialize, Deserialize)]
pub struct CharaParams {
    pub level: u32,
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
            template: CharaTemplateIdx(0),
            inventory: Inventory::for_chara(),
            wait_time: 100.0,
            hp: 100,
            rel: Relationship::NEUTRAL,
        }
    }
}

impl Default for CharaParams {
    fn default() -> CharaParams {
        CharaParams {
            level: 0,
            max_hp: 100,
            str: 0,
            vit: 0,
            dex: 0,
            int: 0,
            wil: 0,
            cha: 0,
            spd: 100,
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
}

