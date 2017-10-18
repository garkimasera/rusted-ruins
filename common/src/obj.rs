
use std::fmt;
use gamedata::item::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ObjectType {
    CharaTemplate, Tile, Wall, AnimImg,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    AnimImg(AnimImgObject),
    CharaTemplate(CharaTemplateObject),
    Item(ItemObject),
    SpecialTile(SpecialTileObject),
    Tile(TileObject),
    UIImg(UIImgObject),
    Wall(WallObject),
}

#[derive(Serialize, Deserialize)]
pub struct AnimImgObject {
    pub id: String,
    pub img: Img,
    pub duration: u32,
    pub n_frame: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CharaTemplateObject {
    pub id: String,
    pub img: Img,
    pub str: u32,
    pub vit: u32,
    pub dex: u32,
    pub int: u32,
    pub wil: u32,
    pub cha: u32,
    pub spd: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SpecialTileObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct TileObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct UIImgObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct WallObject {
    pub id: String,
    pub img: Img,
}

#[derive(Serialize, Deserialize)]
pub struct ItemObject {
    pub id: String,
    pub img: Img,
    pub icon: Icon,
    pub basic_price: f32,
    pub content: ItemContent,
}

#[derive(Serialize, Deserialize)]
pub struct Img {
    pub data: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub grid_w: u32,
    pub grid_h: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// nth image is for icon
    pub n: u32,
}

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
    AnimImgObject, CharaTemplateObject, ItemObject, SpecialTileObject, TileObject,
    UIImgObject, WallObject
);

impl Object {
    pub fn get_id(&self) -> &str {
        match *self {
            Object::AnimImg(ref o) => &o.id,
            Object::CharaTemplate(ref o) => &o.id,
            Object::Item(ref o) => &o.id,
            Object::SpecialTile(ref o) => &o.id,
            Object::Tile(ref o) => &o.id,
            Object::UIImg(ref o) => &o.id,
            Object::Wall(ref o) => &o.id,
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
        let n = if n < img.grid_w * img.grid_h {
            n
        }else{
            1
        };
        let grid_x = n % img.grid_w;
        let grid_y = n / img.grid_h;
        ((img.w * grid_x) as i32, (img.h * grid_y) as i32, img.w, img.h)
    }
}

/// Objects that have icon image
pub trait IconObject: ImgObject {
    /// Which is icon image
    fn which_icon_img(&self) -> u32;
    
    fn icon_img_rect(&self) -> (i32, i32, u32, u32) {
        let rect = self.img_rect_nth(self.which_icon_img());
        (rect.0, rect.1, ::basic::ICON_SIZE, ::basic::ICON_SIZE)
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

macro_rules! impl_icon_object {
    ( $($obj:ty),* ) => {
        $(
            impl IconObject for $obj {
                fn which_icon_img(&self) -> u32 {
                    self.icon.n
                }
            }
        )*
    }
}

impl_img_object!(
    AnimImgObject, CharaTemplateObject, ItemObject, SpecialTileObject, TileObject,
    UIImgObject, WallObject
);

impl_icon_object!(
    ItemObject
);

