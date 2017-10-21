
use common::gamedata;

#[derive(Debug, Deserialize)]
pub struct TomlInput {
    pub object_type: String,
    pub id: String,
    pub image: Option<ImgInput>,
    pub icon: Option<IconInput>,
    pub anim_img: Option<AnimImgDepInput>,
    pub chara_template: Option<CharaTemplateDepInput>,
    pub item: Option<ItemDepInput>,
    pub tile: Option<TileDepInput>,
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
    pub w: u32,
    pub h: u32,
    pub grid_w: Option<u32>,
    pub grid_h: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct IconInput {
    pub n: Option<u32>,
}

// Type dependent fields

#[derive(Debug, Deserialize)]
pub struct AnimImgDepInput {
    pub duration: u32,
    pub n_frame: u32,
}

#[derive(Debug, Deserialize)]
pub struct CharaTemplateDepInput {
    pub race: gamedata::chara::Race,
    pub gen_weight: u32,
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
}

#[derive(Debug, Deserialize)]
pub struct ItemDepInput {
    pub item_kind: String,
    pub basic_price: f32,
    pub eff: Option<f32>,
}

