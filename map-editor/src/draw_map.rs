use crate::edit_map::EditingMap;
use crate::pixbuf_holder::PixbufHolder;
use cairo::Context;
use common::basic::{N_TILE_IMG_LAYER, PIECE_SIZE_I, TILE_SIZE_I};
use common::gobj;
use common::obj::ImgObject;
use common::objholder::*;
use common::piece_pattern::*;
use gdk::prelude::GdkContextExt;
use gdk_pixbuf::Pixbuf;
use geom::*;

macro_rules! try_paint {
    ($result:expr) => {
        if let Err(e) = $result {
            log::warn!("{:?}", e);
        }
    };
}

/// Draw tiles and objects on map
#[allow(clippy::needless_range_loop)]
pub fn draw_map(
    cr: &Context,
    map: &EditingMap,
    pbh: &PixbufHolder,
    width: i32,
    height: i32,
    pos: (i32, i32),
    layer_visible: [bool; N_TILE_IMG_LAYER],
    wall_visible: bool,
    deco_visible: bool,
    item_visible: bool,
) {
    let tile_nx = width / TILE_SIZE_I + 1;
    let tile_ny = height / TILE_SIZE_I + 1;

    // Clear drawing area
    cr.set_source_rgb(0.5, 0.5, 0.5);
    try_paint!(cr.paint());

    for iy in 0..tile_ny {
        for ix in 0..tile_nx {
            let p = Vec2d(ix + pos.0, iy + pos.1);
            if p.0 >= map.width as i32 || p.1 >= map.height as i32 {
                continue;
            }

            for i_tile in 0..N_TILE_IMG_LAYER {
                if !layer_visible[i_tile] {
                    continue;
                }
                if let Some((idx, pp)) = map.tile[p][i_tile].get() {
                    // Draw tile
                    draw_pieces(cr, pbh, idx, pp, ix, iy);
                }
            }

            // Draw wall
            if wall_visible && !map.wall[p].is_empty() {
                draw_wall_pieces(
                    cr,
                    pbh,
                    map.wall[p].idx().unwrap(),
                    map.wall[p].piece_pattern(),
                    ix,
                    iy,
                );
            }

            // Draw deco
            if deco_visible {
                if let Some(deco_idx) = map.deco[p] {
                    let pixbuf = &pbh.get(deco_idx).image;
                    let height = pixbuf.height();
                    cr.set_source_pixbuf(
                        pixbuf,
                        (ix * TILE_SIZE_I) as f64,
                        (iy * TILE_SIZE_I - height + TILE_SIZE_I) as f64,
                    );
                    try_paint!(cr.paint());
                }
            }

            // Draw item
            if item_visible {
                for item in &map.items[p] {
                    let item_idx: ItemIdx = gobj::id_to_idx(&item.id);
                    let item_obj = gobj::get_obj(item_idx);
                    let pixbuf = &pbh.get(item_idx).image;
                    let (x, y, w, h) = item_obj.img_rect_nth(0);
                    image_copy(
                        cr,
                        pixbuf,
                        x,
                        y,
                        ix * TILE_SIZE_I,
                        iy * TILE_SIZE_I - h as i32 + TILE_SIZE_I,
                        w,
                        h,
                    );
                }
            }
        }
    }
}

fn draw_pieces(
    cr: &Context,
    pbh: &PixbufHolder,
    idx: TileIdx,
    piece_pattern: PiecePattern,
    ix: i32,
    iy: i32,
) {
    let image = &pbh.get(idx).image;
    let tile_obj = gobj::get_obj(idx);

    // Top left piece
    if let Some(rect) = tile_obj.piece_rect(piece_pattern.top_left, 0, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I,
            iy * TILE_SIZE_I,
            rect.2,
            rect.3,
        );
    }
    // Top right piece
    if let Some(rect) = tile_obj.piece_rect(piece_pattern.top_right, 1, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I + PIECE_SIZE_I,
            iy * TILE_SIZE_I,
            rect.2,
            rect.3,
        );
    }
    // Bottom left piece
    if let Some(rect) = tile_obj.piece_rect(piece_pattern.bottom_left, 2, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I,
            iy * TILE_SIZE_I + PIECE_SIZE_I,
            rect.2,
            rect.3,
        );
    }
    // Bottom right piece
    if let Some(rect) = tile_obj.piece_rect(piece_pattern.bottom_right, 3, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I + PIECE_SIZE_I,
            iy * TILE_SIZE_I + PIECE_SIZE_I,
            rect.2,
            rect.3,
        );
    }
}

fn draw_wall_pieces(
    cr: &Context,
    pbh: &PixbufHolder,
    idx: WallIdx,
    piece_pattern: PiecePattern,
    ix: i32,
    iy: i32,
) {
    let image = &pbh.get(idx).image;
    let wall_obj = gobj::get_obj(idx);
    let h = wall_obj.get_img().h as i32 - TILE_SIZE_I;

    // Top left piece
    if let Some(rect) = wall_obj.piece_rect(piece_pattern.top_left, 0, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I,
            iy * TILE_SIZE_I - h,
            rect.2,
            rect.3,
        );
    }
    // Top right piece
    if let Some(rect) = wall_obj.piece_rect(piece_pattern.top_right, 1, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I + PIECE_SIZE_I,
            iy * TILE_SIZE_I - h,
            rect.2,
            rect.3,
        );
    }
    // Bottom left piece
    if let Some(rect) = wall_obj.piece_rect(piece_pattern.bottom_left, 2, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I,
            iy * TILE_SIZE_I + PIECE_SIZE_I - h,
            rect.2,
            rect.3,
        );
    }
    // Bottom right piece
    if let Some(rect) = wall_obj.piece_rect(piece_pattern.bottom_right, 3, 0) {
        image_copy(
            cr,
            image,
            rect.0,
            rect.1,
            ix * TILE_SIZE_I + PIECE_SIZE_I,
            iy * TILE_SIZE_I + PIECE_SIZE_I - h,
            rect.2,
            rect.3,
        );
    }
}

pub fn image_copy(
    cr: &Context,
    pixbuf: &Pixbuf,
    src_x: i32,
    src_y: i32,
    dest_x: i32,
    dest_y: i32,
    w: u32,
    h: u32,
) {
    let w = w as i32;
    let h = h as i32;

    cr.set_source_pixbuf(pixbuf, (dest_x - src_x) as f64, (dest_y - src_y) as f64);
    cr.rectangle(dest_x as f64, dest_y as f64, w as f64, h as f64);
    try_paint!(cr.fill());
}
