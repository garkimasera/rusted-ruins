
use std::collections::HashMap;
use common::gamedata::CharaClass;

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct Chara {
    /// Attribute revisions by class
    pub class_revision: HashMap<CharaClass, CharaAttrRevision>,
    /// Default value of CharaParams::view_range.
    /// The actual value will be adjusted by character traits, and map attributes, etc.
    pub default_view_range: i32,
    /// Default sp when a new character is created.
    pub default_sp: i32,
    /// Character's sp is decreased by this value per turn.
    pub sp_consumption: i32,
    /// sp border of hungry
    pub sp_hungry: i32,
    /// sp border of weak
    pub sp_weak: i32,
    /// sp border of starving
    pub sp_starving: i32,
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
}

impl CharaBaseAttr {
    pub fn revise(self, r: CharaAttrRevision) -> CharaBaseAttr {
        CharaBaseAttr {
            base_hp: self.base_hp + r.hp,
            str: self.str + r.str,
            vit: self.vit + r.vit,
            dex: self.dex + r.dex,
            int: self.int + r.int,
            wil: self.wil + r.wil,
            cha: self.cha + r.cha,
            spd: self.spd + r.spd,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CharaAttrRevision {
    pub hp: i32,
    pub str: i16,
    pub vit: i16,
    pub dex: i16,
    pub int: i16,
    pub wil: i16,
    pub cha: i16,
    pub spd: i16,
}

