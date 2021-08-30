use crate::gamedata;
use crate::gamedata::CharaBaseAttr;
use crate::hashmap::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ObjectType {
    CharaTemplate,
    Tile,
    Wall,
    AnimImg,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    AnimImg(AnimImgObject),
    CharaTemplate(CharaTemplateObject),
    Deco(DecoObject),
    EffectImg(EffectImgObject),
    Item(ItemObject),
    SpecialTile(SpecialTileObject),
    Tile(TileObject),
    UiImg(UiImgObject),
    Wall(WallObject),
    MapTemplate(MapTemplateObject),
    RegionGen(RegionGenObject),
    Script(ScriptObject),
    SiteGen(SiteGenObject),
}

#[derive(Serialize, Deserialize)]
pub struct AnimImgObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct DecoObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct EffectImgObject {
    pub id: String,
    pub img: Img,
}

/// Object that include script data.
#[derive(Serialize, Deserialize)]
pub struct ScriptObject {
    pub id: String,
    pub script: String,
    // pub byte_code: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct SpecialTileObject {
    pub id: String,
    pub always_background: bool,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct TileObject {
    pub id: String,
    pub img: Img,
    pub kind: TileKind,
    pub symbol_color: (u8, u8, u8),
    pub fertility: u8,
    /// Needed skill level to build this tile
    pub build_skill: Option<u32>,
    /// Needed materials to build this tile
    pub materials: Vec<(String, u32)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileKind {
    Ground,
    Water,
}

#[derive(Serialize, Deserialize)]
pub struct UiImgObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct WallObject {
    pub id: String,
    pub hp: u16,
    /// If this is false, skips base tile drawing
    pub base_draw: bool,
    pub img: Img,
    pub symbol_color: (u8, u8, u8),
    /// Needed skill level to build this wall
    pub build_skill: Option<u32>,
    /// Needed materials to build this wall
    pub materials: Vec<(String, u32)>,
    /// Rewards after mining this wall
    pub mining_rewards: Vec<(String, u32)>,
}

pub use crate::gamedata::chara::CharaTemplateObject;
pub use crate::gamedata::item::ItemObject;

#[derive(Serialize, Deserialize)]
pub struct Img {
    pub data: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub grid_nx: u32,
    pub grid_ny: u32,
    pub n_frame: u32,
    /// Number of image pattern. Used for tile piece processing or image variation.
    pub n_pattern: u32,
    pub n_anim_frame: u32,
    pub duration: u32,
    pub variation_rule: ImgVariationRule,
}

/// Image variation rule.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ImgVariationRule {
    /// No variation.
    None,
    /// Random on generation this object.
    RandomOnGen,
    /// Growing plants
    Growing,
}

impl Default for ImgVariationRule {
    fn default() -> Self {
        ImgVariationRule::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// nth image is for icon
    pub n: u32,
}

// No image objects

pub use crate::maptemplate::MapTemplateObject;
pub use crate::regiongen::RegionGenObject;
pub use crate::sitegen::SiteGenObject;

macro_rules! impl_object {
    ( $($i:ty),* ) => {
        $(
            impl fmt::Debug for $i {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{} {{ id: \"{}\" }}", stringify!($i), self.id)
                }
            }
        )*
    }
}

impl_object!(
    AnimImgObject,
    CharaTemplateObject,
    DecoObject,
    EffectImgObject,
    ItemObject,
    SpecialTileObject,
    TileObject,
    UiImgObject,
    WallObject,
    MapTemplateObject,
    RegionGenObject,
    SiteGenObject,
    ScriptObject
);

impl Object {
    pub fn get_id(&self) -> &str {
        match *self {
            Object::AnimImg(ref o) => &o.id,
            Object::CharaTemplate(ref o) => &o.id,
            Object::Deco(ref o) => &o.id,
            Object::EffectImg(ref o) => &o.id,
            Object::Item(ref o) => &o.id,
            Object::SpecialTile(ref o) => &o.id,
            Object::Tile(ref o) => &o.id,
            Object::UiImg(ref o) => &o.id,
            Object::Wall(ref o) => &o.id,
            Object::MapTemplate(ref o) => &o.id,
            Object::RegionGen(ref o) => &o.id,
            Object::Script(ref o) => &o.id,
            Object::SiteGen(ref o) => &o.id,
        }
    }
}

/// Objects that have image
pub trait ImgObject {
    fn get_img(&self) -> &Img;
    /// Returns rect for the first image
    fn img_rect(&self) -> (i32, i32, u32, u32) {
        let img = self.get_img();
        (0, 0, img.w, img.h)
    }

    /// Returns rect for nth image of grid
    fn img_rect_nth(&self, n: u32) -> (i32, i32, u32, u32) {
        let img = self.get_img();
        let n = if n < img.grid_nx * img.grid_ny { n } else { 0 };
        let grid_x = n % img.grid_nx;
        let grid_y = n / img.grid_nx;
        (
            (img.w * grid_x) as i32,
            (img.h * grid_y) as i32,
            img.w,
            img.h,
        )
    }

    /// Returns rect for specified pattern
    fn img_rect_pattern(&self, i_pattern: u32) -> (i32, i32, u32, u32) {
        let img = self.get_img();
        self.img_rect_nth(img.n_anim_frame * i_pattern)
    }
}

macro_rules! impl_img_object {
    ( $($obj:ty),* ) => {
        $(
            impl ImgObject for $obj {
                fn get_img(&self) -> &Img {
                    &self.img
                }
            }
        )*
    }
}

impl_img_object!(
    AnimImgObject,
    EffectImgObject,
    CharaTemplateObject,
    DecoObject,
    ItemObject,
    SpecialTileObject,
    TileObject,
    UiImgObject,
    WallObject
);
