//! map_drawing_area functions

use common::basic::TILE_SIZE;
use cairo::Context;

/// Draw tiles and objects on map
pub fn draw_map(context: &Context, width: i32, height: i32) {
    let tile_nx = width / TILE_SIZE as i32 + 1;
    let tile_ny = height / TILE_SIZE as i32 + 1;
}

