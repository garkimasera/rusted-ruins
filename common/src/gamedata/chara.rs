
use objholder::CharaTemplateIdx;
use gamedata::item::Inventory;

/// Relationship between one chara to another.
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
            (HOSTILE, NEUTRAL) => NEUTRAL, (HOSTILE, HOSTILE) => HOSTILE,
        }
    }
}

/// All data for one character
#[derive(Serialize, Deserialize)]
pub struct Chara {
    pub name: String,
    pub params: CharaParams,
    pub template: String,
    /// CharaTemplateIdx is preserved for use in CharaHolder
    /// But ignored when it is saved
    pub template_idx: CharaTemplateIdx,
    pub inventory: Inventory,
    pub wait_time: f32,
    /// Relationship to player character
    pub rel: Relationship,
}

#[derive(Serialize, Deserialize)]
pub struct CharaParams {
    pub level: u32,
    pub max_hp: i32,
    pub hp:  i32,
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
            template: "!!".to_owned(),
            template_idx: CharaTemplateIdx(0),
            inventory: Inventory::for_chara(),
            wait_time: 100.0,
            rel: Relationship::NEUTRAL,
        }
    }
}

impl Default for CharaParams {
    fn default() -> CharaParams {
        CharaParams {
            level: 0,
            max_hp: 100,
            hp: 100,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CharaId {
    /// Player is unique character in the game
    Player,
    /// Indexed for a map. This character don't appear on other maps
    OnMap(::gamedata::map::MapId),
}


