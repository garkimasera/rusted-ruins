
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use common::obj::Img;
use common::objholder::*;
use common::gobj;

macro_rules! impl_pixbuf_holder {
    ($({$mem:ident, $idx:ty}),*) => {
        // Owns all SDL texture
        pub struct PixbufHolder {
            $(pub $mem: Vec<Pixbuf>),*
        }

        impl PixbufHolder {
            pub fn new() -> PixbufHolder {
                let objholder = gobj::get_objholder();
                
                let mut pbh = PixbufHolder {
                    $($mem: Vec::new()),*
                };

                $(
                    for ref o in &objholder.$mem {
                        let pixbuf = load_png(&o.img);
                        pbh.$mem.push(pixbuf);
                    }
                )*
                
                pbh
            }
        }

        $(
            impl Holder<$idx> for PixbufHolder {
                type ReturnType = Pixbuf;
                fn get(&self, idx: $idx) -> &Pixbuf {
                    &self.$mem[idx.0 as usize]
                }
            }
        )*
    }
}

impl_pixbuf_holder! {
    {deco, DecoIdx},
    {chara_template, CharaTemplateIdx},
    {special_tile, SpecialTileIdx},
    {tile, TileIdx},
    {wall, WallIdx}
}

fn load_png(img: &Img) -> Pixbuf {
    let loader = PixbufLoader::new_with_type("png").unwrap();
    loader.loader_write(&img.data).unwrap();
    loader.close().unwrap();
    let pixbuf = loader.get_pixbuf().unwrap();
    if img.grid_w == 1 && img.grid_h == 1 {
        pixbuf
    } else {
        pixbuf.new_subpixbuf(0, 0, img.w as i32, img.h as i32)
    }
}
