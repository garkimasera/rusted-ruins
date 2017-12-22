
use common::basic::TILE_SIZE_I;
use common::objholder::*;
use cairo::Context;
use gdk::prelude::ContextExt;
use pixbuf_holder::PixbufHolder;

/// Draw tiles and objects on map
pub fn draw_map(context: &Context, pbh: &PixbufHolder, width: i32, height: i32) {
    let tile_nx = width / TILE_SIZE_I + 1;
    let tile_ny = height / TILE_SIZE_I + 1;

    for iy in 0..tile_ny {
        for ix in 0..tile_nx {
            context.set_source_pixbuf(pbh.get(TileIdx(1)),
                                      (ix * TILE_SIZE_I) as f64,
                                      (iy * TILE_SIZE_I) as f64);
            context.paint();
        }
    }
    
}

