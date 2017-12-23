
use gdk_pixbuf::{Pixbuf, PixbufLoader};
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
                        let pixbuf = load_png(&o.img.data);
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
    {chara_template, CharaTemplateIdx},
    {special_tile, SpecialTileIdx},
    {tile, TileIdx},
    {wall, WallIdx}
}

fn load_png(data: &[u8]) -> Pixbuf {
    let loader = PixbufLoader::new_with_type("png").unwrap();
    loader.loader_write(data).unwrap();
    loader.close().unwrap();
    loader.get_pixbuf().unwrap()
}
