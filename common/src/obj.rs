use crate::gamedata;
use crate::gamedata::CharaBaseAttr;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ObjectType {
    CharaTemplate,
    Tile,
    Wall,
    AnimImg,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    AnimImg(AnimImgObject),
    CharaTemplate(CharaTemplateObject),
    Deco(DecoObject),
    Effect(EffectObject),
    Item(ItemObject),
    SpecialTile(SpecialTileObject),
    Tile(TileObject),
    UIImg(UIImgObject),
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
pub struct CharaTemplateObject {
    pub id: String,
    pub img: Img,
    /// Character's race
    pub race: gamedata::Race,
    /// The frequency of character generation for random map
    pub gen_weight: f32,
    /// Generation level
    /// If it is higher, and the character will be generated on deeper floors
    pub gen_level: u32,
    /// Default AI kind for this character
    pub default_ai_kind: gamedata::NpcAIKind,
    pub base_attr: CharaBaseAttr,
}

#[derive(Serialize, Deserialize)]
pub struct DecoObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct EffectObject {
    pub id: String,
    pub img: Img,
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TileKind {
    Ground,
    Water,
}

#[derive(Serialize, Deserialize)]
pub struct UIImgObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct WallObject {
    pub id: String,
    /// If this is false, skips base tile drawing
    pub base_draw: bool,
    pub img: Img,
    pub symbol_color: (u8, u8, u8),
    pub build_skill: Option<u32>,
    pub materials: Option<Vec<(String, u32)>>,
}

pub use crate::gamedata::item::ItemObject;

#[derive(Serialize, Deserialize)]
pub struct Img {
    pub data: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub grid_nx: u32,
    pub grid_ny: u32,
    pub n_frame: u32,
    pub n_pattern: u32,
    pub n_anim_frame: u32,
    pub duration: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// nth image is for icon
    pub n: u32,
}

// No image objects

pub use crate::maptemplate::MapTemplateObject;
pub use crate::regiongen::RegionGenObject;
pub use crate::script::ScriptObject;
pub use crate::sitegen::SiteGenObject;

macro_rules! impl_object {
    ( $($i:ty),* ) => {
        $(
            impl fmt::Debug for $i {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    EffectObject,
    ItemObject,
    SpecialTileObject,
    TileObject,
    UIImgObject,
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
            Object::Effect(ref o) => &o.id,
            Object::Item(ref o) => &o.id,
            Object::SpecialTile(ref o) => &o.id,
            Object::Tile(ref o) => &o.id,
            Object::UIImg(ref o) => &o.id,
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
    EffectObject,
    CharaTemplateObject,
    DecoObject,
    ItemObject,
    SpecialTileObject,
    TileObject,
    UIImgObject,
    WallObject
);
