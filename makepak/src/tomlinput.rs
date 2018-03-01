
use array2d::Vec2d;
use std::collections::HashMap;
use common::gamedata;
use common::talkscript;
use common::sitegen;

#[derive(Debug, Deserialize)]
pub struct TomlInput {
    pub object_type: String,
    pub id: String,
    pub image: Option<ImgInput>,
    pub chara_template: Option<CharaTemplateDepInput>,
    pub item: Option<ItemDepInput>,
    pub tile: Option<TileDepInput>,
    pub wall: Option<WallDepInput>,
    pub special_tile: Option<SpecialTileDepInput>,
    pub region_gen: Option<RegionGenDepInput>,
    pub site_gen: Option<SiteGenDepInput>,
    pub talk_script: Option<TalkScriptDepInput>,
}

/// If tomlinput has specified optional field, return it. If not, return error.
macro_rules! get_optional_field {
    ($tomlinput:expr, $field:ident) => {
        match $tomlinput.$field {
            Some(i) => i,
            None => bail!($crate::error::ErrorKind::MissingField(stringify!($field).into())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImgInput {
    pub path: String,
    pub w: Option<u32>,
    pub h: Option<u32>,
    pub grid_w: Option<u32>,
    pub grid_h: Option<u32>,
    pub n_frame: Option<u32>,
    pub duration: Option<u32>,
}

// Type dependent fields

#[derive(Debug, Deserialize)]
pub struct CharaTemplateDepInput {
    pub race: gamedata::chara::Race,
    pub gen_weight: f32,
    pub gen_level: u32,
    pub max_hp: i32,
    pub str: u32,
    pub vit: u32,
    pub dex: u32,
    pub int: u32,
    pub wil: u32,
    pub cha: u32,
    pub spd: u32, 
}

#[derive(Debug, Deserialize)]
pub struct TileDepInput {
    pub kind: ::common::obj::TileKind,
}

#[derive(Debug, Deserialize)]
pub struct WallDepInput {
    pub base_draw: Option<bool>,
    pub always_background: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialTileDepInput {
    pub always_background: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ItemDepInput {
    pub item_kind: String,
    pub basic_price: u32,
    pub gen_weight: f32,
    pub gen_level: u32,
    pub eff: Option<i32>,
    pub dice_n: Option<u16>,
    pub dice_x: Option<u16>,
    pub weapon_kind: Option<gamedata::item::WeaponKind>,
    pub armor_kind: Option<gamedata::item::ArmorKind>,
    pub medical_effect: Option<gamedata::item::MedicalEffect>,
    /// For armor items
    pub def: Option<u16>,
    /// For armor items
    pub mdf: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct TalkScriptDepInput {
    pub sections: HashMap<String, TalkScriptSectionInput>,
}

#[derive(Debug, Deserialize)]
pub struct RegionGenDepInput {
    pub map_template_id: String,
    pub towns: Vec<SiteGenIdAndPos>,
}

#[derive(Debug, Deserialize)]
pub struct SiteGenIdAndPos {
    pub id: String,
    pub pos: Vec2d,
}

#[derive(Debug, Deserialize)]
pub struct SiteGenDepInput {
    pub kind: gamedata::site::SiteKind,
    pub map_template_id: Vec<String>,
    pub unique_citizens: Option<Vec<sitegen::UniqueCitizenGenData>>,
    pub shops: Option<Vec<sitegen::ShopGenData>>,
}

#[derive(Debug, Deserialize)]
pub struct TalkScriptSectionInput {
    pub is_empty: Option<bool>,
    pub text: Option<String>,
    pub reaction: talkscript::TalkSectionReaction,
    pub sub_reaction: Option<Vec<talkscript::TalkSubReaction>>,
}

