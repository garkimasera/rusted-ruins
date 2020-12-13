use common::gamedata::{
    self, Element, ElementArray, FactionId, Harvest, SkillKind, StatusEffect, TargetMode,
};
use common::sitegen;
use geom::{ShapeKind, Vec2d};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub object_type: String,
    pub id: String,
    pub image: Option<ImgInput>,
    pub chara_template: Option<CharaTemplateDepInput>,
    pub item: Option<ItemDepInput>,
    pub tile: Option<TileDepInput>,
    pub wall: Option<WallDepInput>,
    pub special_tile: Option<SpecialTileDepInput>,
    pub region_gen: Option<RegionGenDepInput>,
    pub script: Option<ScriptDepInput>,
    pub site_gen: Option<SiteGenDepInput>,
}

/// If tomlinput has specified optional field, return it. If not, return error.
macro_rules! get_optional_field {
    ($tomlinput:expr, $field:ident) => {
        match $tomlinput.$field {
            Some(i) => i,
            None => anyhow::bail!($crate::error::PakCompileError::MissingField {
                field_name: stringify!($field).into()
            }),
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImgInput {
    pub path: String,
    /// Name of the creator and other copyright information.
    pub copyright: Option<String>,
    pub w: Option<u32>,
    pub h: Option<u32>,
    pub grid_nx: Option<u32>,
    pub grid_ny: Option<u32>,
    pub n_frame: Option<u32>,
    pub n_pattern: Option<u32>,
    pub n_anim_frame: Option<u32>,
    pub duration: Option<u32>,
}

// Type dependent fields

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharaTemplateDepInput {
    pub race: String,
    pub gen_weight: f32,
    pub gen_level: u32,
    pub default_ai_kind: Option<gamedata::NpcAIKind>,
    #[serde(default)]
    pub skill_bonus: HashMap<String, gamedata::SkillBonus>,
    pub base_hp: i32,
    pub str: u16,
    pub vit: u16,
    pub dex: u16,
    pub int: u16,
    pub wil: u16,
    pub cha: u16,
    pub spd: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileDepInput {
    pub kind: ::common::obj::TileKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WallDepInput {
    pub hp: Option<u16>,
    pub base_draw: Option<bool>,
    pub build_skill: Option<u32>,
    pub materials: Option<Vec<(String, u32)>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecialTileDepInput {
    pub always_background: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemDepInput {
    pub item_kind: String,
    pub group: String,
    pub basic_price: u32,
    pub w: u32,
    pub gen_weight: f32,
    pub shop_weight: Option<f32>,
    pub gen_level: u32,
    pub eff: Option<u16>,
    pub dice_n: Option<u16>,
    pub dice_x: Option<u16>,
    pub weapon_kind: Option<gamedata::WeaponKind>,
    pub armor_kind: Option<gamedata::ArmorKind>,
    #[serde(default)]
    pub medical_effect: Option<EffectInput>,
    #[serde(default)]
    pub magical_effect: Option<EffectInput>,
    #[serde(default)]
    pub use_effect: Option<EffectInput>,
    #[serde(default)]
    pub tool_effect: gamedata::ToolEffect,
    /// For armor items
    pub def: Option<ElementArray<u16>>,
    pub nutrition: Option<u16>,
    #[serde(default)]
    pub charge: [u8; 2],
    pub harvest: Option<Harvest>,
    pub facility: Option<(String, i8)>,
    #[serde(default)]
    pub material_group: String,
    #[serde(default)]
    pub titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionGenDepInput {
    pub map_template_id: String,
    pub towns: Vec<SiteGenIdAndPos>,
    pub others: Vec<SiteGenIdAndPos>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteGenIdAndPos {
    pub id: String,
    pub pos: Vec2d,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteGenDepInput {
    pub kind: gamedata::site::SiteKind,
    pub site_symbol: gamedata::map::SiteSymbolKind,
    pub map_template_id: Vec<String>,
    pub default_faction_id: FactionId,
    pub unique_citizens: Option<Vec<sitegen::UniqueCitizenGenData>>,
    pub shops: Option<Vec<sitegen::ShopGenData>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptDepInput {
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectInput {
    pub kind: Vec<EffectKindInput>,
    #[serde(default)]
    pub target_mode: TargetMode,
    #[serde(default)]
    pub power_adjust: Vec<f32>,
    #[serde(default)]
    pub range: u32,
    #[serde(default)]
    pub shape: ShapeKind,
    #[serde(default)]
    pub size: u32,
    #[serde(default)]
    pub anim_img: String,
    #[serde(default)]
    pub anim_img_shot: String,
    #[serde(default)]
    pub sound: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectKindInput {
    pub kind: String,
    pub element: Option<Element>,
    pub status: Option<StatusEffect>,
    pub skills: Option<Vec<SkillKind>>,
}
