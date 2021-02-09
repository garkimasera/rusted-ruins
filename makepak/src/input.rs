use common::gamedata::{self, Effect, ElementArray, FactionId, Harvest};
use common::sitegen;
use geom::Vec2d;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub object_type: String,
    pub id: String,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub image: Option<ImgInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub chara_template: Option<CharaTemplateDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub item: Option<ItemDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub tile: Option<TileDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub wall: Option<WallDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub special_tile: Option<SpecialTileDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub region_gen: Option<RegionGenDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub script: Option<ScriptDepInput>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub site_gen: Option<SiteGenDepInput>,
}

/// If input has specified optional field, return it. If not, return error.
macro_rules! get_optional_field {
    ($input:expr, $field:ident) => {
        match $input.$field {
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
    #[serde(default)]
    pub copyright: String,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub w: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub h: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub grid_nx: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub grid_ny: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub n_frame: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub n_pattern: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub n_anim_frame: Option<u32>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub duration: Option<u32>,
    #[serde(default)]
    pub variation_rule: common::obj::ImgVariationRule,
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
    #[serde(default)]
    pub mining_rewards: Vec<(String, u32)>,
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
    #[serde(default)]
    pub quality_kind: gamedata::QualityKind,
    pub gen_weight: f32,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub shop_weight: Option<f32>,
    pub gen_level: u32,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub eff: Option<u16>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub eff_var: Option<u16>,
    /// For armor items
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub def: Option<ElementArray<u16>>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub weapon_kind: Option<gamedata::WeaponKind>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub armor_kind: Option<gamedata::ArmorKind>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub throw_effect: Option<Effect>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub medical_effect: Option<Effect>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub magical_effect: Option<Effect>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub use_effect: Option<gamedata::UseEffect>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub tool_effect: Option<gamedata::ToolEffect>,
    #[serde(default)]
    pub attrs: Vec<gamedata::ItemObjAttr>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub harvest: Option<Harvest>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub facility: Option<(String, i8)>,
    #[serde(default)]
    pub material_group: String,
    #[serde(default)]
    pub material: gamedata::MaterialName,
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
