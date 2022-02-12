use common::basic::BonusLevel;
use common::gamedata::{self, AbilityId, CharaClass, EquipGen, FactionId, SkillKind};
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
    pub ui_img: Option<UiImgDepInput>,
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
    #[serde(default)]
    pub class: CharaClass,
    #[serde(default)]
    pub faction: FactionId,
    pub gen_weight: f32,
    pub gen_level: u32,
    #[serde(default)]
    pub default_ai_kind: gamedata::NpcAiKind,
    #[serde(default)]
    pub skill_bonus: HashMap<SkillKind, BonusLevel>,
    #[serde(default)]
    pub abilities: Vec<AbilityId>,
    #[serde(default)]
    pub equips: Vec<EquipGen>,
    pub base_hp: i32,
    pub str: u16,
    pub vit: u16,
    pub dex: u16,
    pub int: u16,
    pub wil: u16,
    pub cha: u16,
    pub spd: u16,
    pub carry: u16,
    pub travel_speed: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileDepInput {
    pub kind: ::common::obj::TileKind,
    #[serde(default)]
    pub fertility: u8,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub build_skill: Option<u32>,
    #[serde(default)]
    pub materials: Vec<(String, u32)>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiImgDepInput {
    #[serde(default)]
    pub hot: (u8, u8),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WallDepInput {
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub hp: Option<u16>,
    pub base_draw: Option<bool>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub build_skill: Option<u32>,
    #[serde(default)]
    pub materials: Vec<(String, u32)>,
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
    pub weapon_kind: Option<gamedata::WeaponKind>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub armor_kind: Option<gamedata::ArmorKind>,
    #[serde(default)]
    pub attrs: Vec<gamedata::ItemObjAttr>,
    #[serde(default)]
    pub material_group: String,
    #[serde(default)]
    pub material: gamedata::MaterialName,
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
    #[serde(default)]
    pub npcs: Vec<sitegen::NpcGenData>,
    #[serde(default)]
    pub shops: Vec<sitegen::ShopGenData>,
    #[serde(default)]
    pub quests: Vec<sitegen::QuestGenData>,
    #[serde(default, with = "::serde_with::rust::unwrap_or_skip")]
    pub delivery_chest: Option<(u32, Vec2d, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptDepInput {
    pub script: String,
}
