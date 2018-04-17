
use array2d::*;
use common::basic::TILE_SIZE_I;
use common::objholder::*;
use cairo::Context;
use gdk::prelude::ContextExt;
use gdk_pixbuf::{Pixbuf, PixbufExt};
use pixbuf_holder::PixbufHolder;
use edit_map::EditingMap;

/// Draw tiles and objects on map
pub fn draw_map(cr: &Context, map: &EditingMap, pbh: &PixbufHolder,
                width: i32, height: i32, pos: (i32, i32)) {
    
    let tile_nx = width / TILE_SIZE_I + 1;
    let tile_ny = height / TILE_SIZE_I + 1;

    // Clear drawing area
    cr.set_source_rgb(0.5, 0.5, 0.5);
    cr.paint();

    for iy in 0..tile_ny {
        for ix in 0..tile_nx {
            let p = Vec2d::new(ix + pos.0, iy + pos.1);
            if p.0 >= map.width as i32 || p.1 >= map.height as i32 { continue; }
            
            // Draw tile
            image_copy(cr, &pbh.get(map.tile[p].idx[0]).image,
                       0, 0,
                       ix * TILE_SIZE_I, iy * TILE_SIZE_I,
                       TILE_SIZE_I, TILE_SIZE_I);
            cr.fill();

            // Draw wall
            if let Some(wall_idx) = map.wall[p] {
                let pixbuf = &pbh.get(wall_idx).image;
                let height = pixbuf.get_height();
                cr.set_source_pixbuf(pixbuf,
                                     (ix * TILE_SIZE_I) as f64,
                                     (iy * TILE_SIZE_I - height + TILE_SIZE_I) as f64);
                cr.paint();
            }

            // Draw deco
            if let Some(deco_idx) = map.deco[p] {
                let pixbuf = &pbh.get(deco_idx).image;
                let height = pixbuf.get_height();
                cr.set_source_pixbuf(pixbuf,
                                     (ix * TILE_SIZE_I) as f64,
                                     (iy * TILE_SIZE_I - height + TILE_SIZE_I) as f64);
                cr.paint();
            }
        }
    }
}

pub fn image_copy(cr: &Context, pixbuf: &Pixbuf,
                  src_x: i32, src_y: i32, dest_x: i32, dest_y: i32, w: i32, h: i32) {
    
    cr.set_source_pixbuf(pixbuf, (dest_x - src_x) as f64, (dest_y - src_y) as f64);
    cr.rectangle(dest_x as f64, dest_y as f64, w as f64, h as f64);
    cr.fill();
}

