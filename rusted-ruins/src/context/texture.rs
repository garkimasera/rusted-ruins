use common::gobj;
use common::obj::ImgObject;
use common::objholder::*;
use sdl2::image::ImageRWops;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::video::WindowContext;

macro_rules! impl_texture_holder {
    ($({$mem:ident, $idx:ty}),*) => {
        // Owns all SDL texture
        pub struct TextureHolder<'a> {
            $(pub $mem: Vec<Texture<'a>>),*
        }

        impl<'a> TextureHolder<'a> {
            pub fn new(
                objholder: &ObjectHolder,
                texture_creator: &'a TextureCreator<WindowContext>) -> TextureHolder<'a> {

                info!("start loading textures");

                let tc = TextureCreatorW::new(texture_creator);

                let mut th = TextureHolder {
                    $($mem: Vec::new()),*
                };

                $(
                    for ref o in &objholder.$mem {
                        let texture = match tc.create_texture_from_data(&o.img.data) {
                            Ok(o) => o,
                            Err(_) => { unimplemented!() },
                        };
                        th.$mem.push(texture);
                    }
                )*

                th
            }
        }

        $(
            impl<'a> Holder<$idx> for TextureHolder<'a> {
                type ReturnType = Texture<'a>;
                fn get(&self, idx: $idx) -> &Texture<'a> {
                    &self.$mem[idx.as_usize()]
                }
            }
        )*
    }
}

impl_texture_holder! {
    {anim_img, AnimImgIdx},
    {chara_template, CharaTemplateIdx},
    {deco, DecoIdx},
    {effect_img, EffectImgIdx},
    {item, ItemIdx},
    {special_tile, SpecialTileIdx},
    {tile, TileIdx},
    {ui_img, UiImgIdx},
    {wall, WallIdx}
}

// A thin wrapper for TextureCreator
pub struct TextureCreatorW<'a>(&'a TextureCreator<WindowContext>);

impl<'a> TextureCreatorW<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> TextureCreatorW<'a> {
        TextureCreatorW(texture_creator)
    }

    fn create_texture_from_data(&self, data: &[u8]) -> Result<Texture<'a>, String> {
        let rwops = RWops::from_bytes(data)?;
        let surface = rwops.load_png()?;

        self.0
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())
    }
}

// Implement icon idx
macro_rules! impl_iconidx {
    ($({$a:ident, $idx:ident}),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum IconIdx {
            $(
                $a { idx: $idx, i_pattern: u32 },
            )*
        }

        impl<'a> TextureHolder<'a> {
            pub fn get_icon(&self, idx: IconIdx) -> (&Texture<'a>, Rect) {
                match idx {
                    $(
                        IconIdx::$a { idx, i_pattern } => {
                            let t = self.get(idx);
                            let r = gobj::get_obj(idx).img_rect_pattern(i_pattern);
                            return (t, Rect::from(r));
                        }
                    )*
                }
            }
        }

        $(
            impl From<$idx> for IconIdx {
                fn from(i: $idx) -> IconIdx { IconIdx::$a { idx: i, i_pattern: 0 } }
            }
        )*
    }
}

impl_iconidx! {
    {Item, ItemIdx},
    {UiImg, UiImgIdx}
}

impl IconIdx {
    pub fn empty() -> Self {
        Self::UiImg {
            idx: gobj::id_to_idx("!"),
            i_pattern: 0,
        }
    }

    pub fn checked() -> Self {
        Self::UiImg {
            idx: gobj::id_to_idx("!icon-ok"),
            i_pattern: 0,
        }
    }

    pub fn ng() -> Self {
        Self::UiImg {
            idx: gobj::id_to_idx("!icon-ng"),
            i_pattern: 0,
        }
    }
}
