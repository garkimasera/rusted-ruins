
use common::objholder::*;
use common::obj::IconObject;
use common::gobj;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::rect::Rect;

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
                let tc = TextureCreatorW::new(texture_creator);

                let mut th = TextureHolder {
                    $($mem: Vec::new()),*
                };

                $(
                    for ref o in &objholder.$mem {
                        let texture = match tc.from_data(&o.img.data) {
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
                    &self.$mem[idx.0 as usize]
                }
            }
        )*
    }
}

impl_texture_holder! {
    {anim_img, AnimImgIdx},
    {chara_template, CharaTemplateIdx},
    {deco, DecoIdx},
    {item, ItemIdx},
    {special_tile, SpecialTileIdx},
    {tile, TileIdx},
    {ui_img, UIImgIdx},
    {wall, WallIdx}
}

// A thin wrapper for TextureCreator
pub struct TextureCreatorW<'a>(&'a TextureCreator<WindowContext>);

impl<'a> TextureCreatorW<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> TextureCreatorW<'a> {
        TextureCreatorW(texture_creator)
    }
    
    fn from_data(&self, data: &[u8]) -> Result<Texture<'a>, String> {
        let rwops = RWops::from_bytes(data)?;
        let surface = rwops.load_png()?;

        Ok(self.0.create_texture_from_surface(surface).map_err(|e| e.to_string())?)
    }
}

// Implement icon idx
macro_rules! impl_iconidx {
    ($({$a:ident, $idx:ident}),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum IconIdx {
            $(
                $a($idx),
            )*
        }

        impl<'a> TextureHolder<'a> {
            pub fn get_icon(&self, idx: IconIdx) -> (&Texture<'a>, Rect) {
                match idx {
                    $(
                        IconIdx::$a(i) => {
                            let t = self.get(i);
                            let r = gobj::get_obj(i).icon_img_rect();
                            return (t, Rect::from(r));
                        }
                    )*
                }
            }
        }

        $(
            impl From<$idx> for IconIdx {
                fn from(i: $idx) -> IconIdx { IconIdx::$a(i) }
            }
        )*
    }
}

impl_iconidx! {
    {Item, ItemIdx}
}

