use common::gobj;
use common::obj::Img;
use common::objholder::*;
use gdk_pixbuf::{Pixbuf, PixbufExt, PixbufLoader, PixbufLoaderExt};

pub struct PixbufSet {
    /// Whole image
    pub image: Pixbuf,
    /// Clipped image used to icon
    pub icon: Pixbuf,
}

macro_rules! impl_pixbuf_holder {
    ($({$mem:ident, $idx:ty}),*) => {
        // Owns Pixbuf data
        pub struct PixbufHolder {
            $(pub $mem: Vec<PixbufSet>),*
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
                type ReturnType = PixbufSet;
                fn get(&self, idx: $idx) -> &PixbufSet {
                    &self.$mem[idx.as_usize()]
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

fn load_png(img: &Img) -> PixbufSet {
    const ERR_MSG: &'static str = "Error occured while loading image";
    let loader = PixbufLoader::new_with_type("png").expect(ERR_MSG);
    loader.write(&img.data).expect(ERR_MSG);
    loader.close().expect(ERR_MSG);
    let pixbuf = loader.get_pixbuf().expect(ERR_MSG);

    let pixbuf_icon = if img.grid_nx == 1 && img.grid_ny == 1 {
        pixbuf.clone()
    } else {
        pixbuf
            .new_subpixbuf(0, 0, img.w as i32, img.h as i32)
            .expect(ERR_MSG)
    };

    PixbufSet {
        image: pixbuf,
        icon: pixbuf_icon,
    }
}
