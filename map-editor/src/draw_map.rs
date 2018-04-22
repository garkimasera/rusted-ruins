
use array2d::*;
use common::basic::{TILE_SIZE_I, PIECE_SIZE_I};
use common::objholder::*;
use common::gobj;
use common::piece_pattern::PieceImgObject;
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

            for i_tile in 0..map.tile[p].len() {
                // Draw tile
                draw_pieces(cr, pbh, map.tile[p].idx[i_tile], map.tile[p].piece_pattern[i_tile], ix, iy);
            }

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

fn draw_pieces(cr: &Context, pbh: &PixbufHolder, idx: TileIdx, piece_pattern: [u8; 4], ix: i32, iy: i32) {
    let image = &pbh.get(idx).image;
    let tile_obj = gobj::get_obj(idx);

    // Top left piece
    let rect = tile_obj.piece_rect(piece_pattern[0], 0, 0);
    image_copy(cr, image,
               rect.0, rect.1,
               ix * TILE_SIZE_I, iy * TILE_SIZE_I,
               rect.2, rect.3);
    cr.fill();
    // Top right piece
    let rect = tile_obj.piece_rect(piece_pattern[1], 1, 0);
    image_copy(cr, image,
               rect.0, rect.1,
               ix * TILE_SIZE_I + PIECE_SIZE_I, iy * TILE_SIZE_I,
               rect.2, rect.3);
    cr.fill();
    // Bottom left piece
    let rect = tile_obj.piece_rect(piece_pattern[2], 2, 0);
    image_copy(cr, image,
               rect.0, rect.1,
               ix * TILE_SIZE_I, iy * TILE_SIZE_I + PIECE_SIZE_I,
               rect.2, rect.3);
    cr.fill();
    // Bottom right piece
    let rect = tile_obj.piece_rect(piece_pattern[3], 3, 0);
    image_copy(cr, image,
               rect.0, rect.1,
               ix * TILE_SIZE_I + PIECE_SIZE_I, iy * TILE_SIZE_I + PIECE_SIZE_I,
               rect.2, rect.3);
    cr.fill();
}

pub fn image_copy(cr: &Context, pixbuf: &Pixbuf,
                  src_x: i32, src_y: i32, dest_x: i32, dest_y: i32, w: u32, h: u32) {
    let w = w as i32;
    let h = h as i32;
    
    cr.set_source_pixbuf(pixbuf, (dest_x - src_x) as f64, (dest_y - src_y) as f64);
    cr.rectangle(dest_x as f64, dest_y as f64, w as f64, h as f64);
    cr.fill();
}

