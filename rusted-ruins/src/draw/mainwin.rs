
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use array2d::*;
use common::basic::{TILE_SIZE, TILE_SIZE_I};
use common::objholder::Holder;
use common::obj::*;
use common::gobj;
use common::gamedata::map::Map;
use common::gamedata::chara::CharaId;
use game::{Game, Animation, InfoGetter};
use sdlvalues::SdlValues;

pub struct MainWinDrawer {
    rect: Rect,
    w: u32, h: u32,
    topleft: Vec2d,
    dx: i32, dy: i32,
}

impl MainWinDrawer {
    pub fn new(rect: Rect) -> MainWinDrawer {
        MainWinDrawer {
            rect: rect,
            w: rect.width(), h: rect.height(),
            topleft: Vec2d::new(0, 0),
            dx: 0, dy: 0,
        }
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, game: &Game, sv: &SdlValues,
                anim: Option<(&Animation, u32)>) {
        super::frame::next_frame();
        let mut player_move_dir = None;

        let player_move_adjust = if let Some(anim) = anim {
            match anim.0 {
                &Animation::PlayerMove{ n_frame, dir } => {
                    let v = dir.as_vec() * (TILE_SIZE_I * (n_frame - anim.1) as i32 / n_frame as i32);
                    player_move_dir = Some(dir);
                    (v.0, v.1)
                },
                _ => (0, 0)
            }
        }else{
            (0, 0)
        };

        let centering_tile = game.gd.player_pos();
        let map = game.gd.get_current_map();
        self.update_draw_params((map.w as i32, map.h as i32),
                                centering_tile, player_move_adjust);
        self.draw_except_anim(canvas, game, sv, player_move_adjust, player_move_dir);

        if let Some(anim) = anim {
            self.draw_anim(canvas, game, sv, anim.0, anim.1);
        }
    }

    fn draw_except_anim(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &SdlValues,
        player_move_adjust: (i32, i32), player_move_dir: Option<Direction>) {

        canvas.set_viewport(self.rect);
        canvas.set_draw_color(Color::RGB(120, 120, 120));

        let gd = &game.gd;
        let map = gd.get_current_map();
        let player_pos = gd.player_pos();
        
        let is_player_moving = player_move_adjust != (0, 0);

        let player_drawing_row = player_pos.1 + if let Some(dir) = player_move_dir {
            if dir.vdir == VDirection::Up { 1 }else{ 0 }
        }else{ 0 };
        
        let tile_range = self.tile_range();

        // Draw ground and special objs for the first row
        let top_row = tile_range.iter1().next().unwrap();
        for nx in tile_range.iter0() {
            let p = Vec2d::new(nx, top_row);
            self.draw_tile_ground(canvas, map, sv, p);
            self.draw_tile_special(canvas, map, sv, p);
        }

        for ny in tile_range.iter1() {
            // Draw ground and special objs for the next row
            for nx in tile_range.iter0() {
                let p = Vec2d::new(nx, ny + 1);
                self.draw_tile_ground(canvas, map, sv, p);
                self.draw_tile_special(canvas, map, sv, p);
                self.draw_item(canvas, map, sv, p);
            }

            // Draw player when moving
            if is_player_moving && ny == player_drawing_row {
                let chara = gd.chara.get(CharaId::Player);
                let ct = gobj::get_obj(chara.template);
                let src = Rect::from(ct.img_rect());
                let dest = self.centering_at_tile(src, player_pos,
                                                  -player_move_adjust.0, -player_move_adjust.1);
                canvas.copy(sv.tex().get(chara.template), src, dest).unwrap();
            }

            for nx in tile_range.iter0() {
                let p = Vec2d::new(nx, ny);
                
                // Draw wall
                self.draw_tile_wall(canvas, map, sv, p);

                if !map.is_inside(p) { continue; }

                // Draw character on the tile
                if let Some(chara_id) = map.get_chara(p) {
                    let chara = gd.chara.get(chara_id);
                    let ct = gobj::get_obj(chara.template);
                    let src = Rect::from(ct.img_rect());
                    
                    if chara_id == CharaId::Player && is_player_moving {
                        continue;
                    };
                    
                    let dest = if chara_id == CharaId::Player {
                        self.centering_at_tile(src, p, -player_move_adjust.0, -player_move_adjust.1)
                    }else{
                        self.centering_at_tile(src, p, 0, 0)
                    };
                    canvas.copy(sv.tex().get(chara.template), src, dest).unwrap();
                }
            }
        }
    }

    /// Draw tile ground
    /// Decoration will be drawed at the same time
    fn draw_tile_ground(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        let tile_idx = map.get_tile_extrapolated(p);
        let o = gobj::get_obj(tile_idx);
        let src = Rect::from(o.img_rect_nth(super::frame::calc_frame(&o.img)));
        let dest = Rect::new(
            p.0 * TILE_SIZE_I + self.dx, p.1 * TILE_SIZE_I + self.dy,
            TILE_SIZE, TILE_SIZE);
        let texture = sv.tex().get(tile_idx);
        check_draw!(canvas.copy(&texture, src, dest));

        self.draw_tile_wall_background(canvas, map, sv, p);
        self.draw_tile_deco(canvas, map, sv, p);
    }

    fn draw_tile_wall(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        let wall_idx = if let Some(wall_idx) = map.get_wall_extrapolated(p) { wall_idx } else { return; };
        let o = gobj::get_obj(wall_idx);
        if o.always_background { return; }
        let src = Rect::from(o.img_rect_nth(super::frame::calc_frame(&o.img)));
        let dest = self.bottom_at_tile(src, p, 0, 0);
        let texture = sv.tex().get(wall_idx);
        check_draw!(canvas.copy(&texture, src, dest));
    }

    /// Draw wall if always_background is true
    fn draw_tile_wall_background(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        let wall_idx = if let Some(wall_idx) = map.get_wall_extrapolated(p) { wall_idx } else { return; };
        let o = gobj::get_obj(wall_idx);
        if !o.always_background { return; }
        let src = Rect::from(o.img_rect_nth(super::frame::calc_frame(&o.img)));
        let dest = self.bottom_at_tile(src, p, 0, 0);
        let texture = sv.tex().get(wall_idx);
        check_draw!(canvas.copy(&texture, src, dest));
        
    }

    fn draw_tile_deco(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        let deco_idx = if let Some(deco_idx) = map.get_deco_extrapolated(p) { deco_idx } else { return; };
        let o = gobj::get_obj(deco_idx);
        let src = Rect::from(o.img_rect_nth(super::frame::calc_frame(&o.img)));
        let dest = self.bottom_at_tile(src, p, 0, 0);
        let texture = sv.tex().get(deco_idx);
        check_draw!(canvas.copy(&texture, src, dest));
    }

    fn draw_tile_special(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        use common::objholder::SpecialTileIdx;
        if !map.is_inside(p) { return; }
        let special_tile_id = map.tile[p].special.obj_id();
        if special_tile_id.is_none() { return; }
        let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id.unwrap());
        let texture = sv.tex().get(special_tile_idx);
        let query = texture.query();
        let src = Rect::new(0, 0, query.width, query.height);
        let dest = self.bottom_at_tile(src, p, 0, 0);
        
        check_draw!(canvas.copy(&texture, src, dest));
    }

    fn draw_item(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        if !map.is_inside(p) { return; }
        if map.tile[p].item_list.is_none() { return; }

        for &(ref item, _) in map.tile[p].item_list.as_ref().unwrap().iter() {
            let texture = sv.tex().get(item.idx);

            let query = texture.query();
            let src = Rect::new(0, 0, query.width, query.height);
            let dest = self.centering_at_tile(src, p, 0, 0);
        
            check_draw!(canvas.copy(&texture, src, dest));
        }
        
    }

    fn draw_anim(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &SdlValues,
                 anim: &Animation, i_frame: u32) {
        match anim {
            &Animation::Img{ idx, range, .. } => {
                for p in range {
                    let src = Rect::from(gobj::get_obj(idx).img_rect_nth(i_frame));
                    let dest = self.centering_at_tile(src, p, 0, 0);
                    check_draw!(canvas.copy(sv.tex().get(idx), src, dest));
                }
            }
            _ => (),
        }
    }

    fn update_draw_params(&mut self,
                          map_size: (i32, i32), centering_tile: Vec2d, player_move_adjust: (i32, i32)) {
        // Center point by pixel
        let center_p = (centering_tile.0 * TILE_SIZE_I + TILE_SIZE_I / 2 - player_move_adjust.0,
                        centering_tile.1 * TILE_SIZE_I + TILE_SIZE_I / 2 - player_move_adjust.1); 
        let (win_w, win_h) = (self.w as i32, self.h as i32);
        let (min_left, min_top) = (0, 0);
        let (max_right, max_bottom) = (map_size.0 * TILE_SIZE_I - 1, map_size.1 * TILE_SIZE_I - 1);
        let left = if center_p.0 - win_w / 2 < min_left {
            min_left
        } else if center_p.0 + win_w / 2 > max_right {
            ::std::cmp::max(max_right - win_w, 0)
        } else {
            center_p.0 - win_w / 2
        };
        let top = if center_p.1 - win_h / 2 < min_top {
            min_top
        } else if center_p.1 + win_h / 2 > max_bottom {
            ::std::cmp::max(max_bottom - win_h, 0)
        } else {
            center_p.1 - win_h / 2
        };
        let top_left_tile = Vec2d::new(left / TILE_SIZE_I, top / TILE_SIZE_I);
        self.dx = -left;
        self.dy = -top;
        self.topleft = top_left_tile;
    }
    
    fn centering_at_tile(&self, src: Rect, tile: Vec2d, dx: i32, dy: i32) -> Rect {
        Rect::new(
            (TILE_SIZE_I * tile.0 + (TILE_SIZE_I - src.w) / 2) + dx + self.dx,
            (TILE_SIZE_I * tile.1 + (TILE_SIZE_I - src.h) / 2) + dy + self.dy,
            src.w as u32, src.h as u32
        )
    }
    
    fn bottom_at_tile(&self, src: Rect, tile: Vec2d, dx: i32, dy: i32) -> Rect {
        Rect::new(
            (TILE_SIZE_I * tile.0 + (TILE_SIZE_I - src.w) / 2) + dx + self.dx,
            tile.1 * TILE_SIZE_I + dy + self.dy + (TILE_SIZE_I - src.h),
            src.w as u32, src.h as u32
        )
    }

    /// Calculate the number of needed tile to fill screen
    fn calc_tile_num(&self) -> (i32, i32) {
        (
            if self.w % TILE_SIZE == 0 { self.w / TILE_SIZE } else { self.w / TILE_SIZE + 1 } as i32,
            if self.h % TILE_SIZE == 0 { self.h / TILE_SIZE } else { self.h / TILE_SIZE + 1 } as i32,
        )
    }

    /// Gets needed range of tiles to draw over the window
    fn tile_range(&self) -> RectIter {
        let (nx, ny) = self.calc_tile_num();
        let top_left = self.topleft;
        let bottom_right = Vec2d::new(nx + top_left.0, ny + top_left.1);
        RectIter::new(top_left, bottom_right)
    }
}

