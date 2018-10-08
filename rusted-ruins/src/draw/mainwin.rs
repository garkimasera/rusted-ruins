
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use array2d::*;
use common::basic::{TILE_SIZE, TILE_SIZE_I, PIECE_SIZE_I, N_TILE_IMG_LAYER};
use common::objholder::{Holder, UIImgIdx};
use common::obj::*;
use common::gobj;
use common::gamedata::*;
use game::{Game, Animation, InfoGetter};
use game::view::ViewMap;
use sdlvalues::SdlValues;
use super::tile_getter::*;
use super::frame::calc_frame;
use super::overlay;

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
                anim: Option<(&Animation, u32)>, centering_tile: Option<Vec2d>) {
        super::frame::next_frame();
        let mut player_move_dir = None;

        let ct = if let Some(ct) = centering_tile {
            ct
        } else {
            game.gd.player_pos()
        };

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

        let map = game.gd.get_current_map();
        self.update_draw_params((map.w as i32, map.h as i32),
                                ct, player_move_adjust);
        self.draw_except_anim(canvas, game, sv, player_move_adjust, player_move_dir);
        self.draw_overlay_all(canvas, game, sv);

        if let Some(anim) = anim {
            self.draw_anim(canvas, game, sv, anim.0, anim.1);
        }

        if centering_tile.is_some() {
            self.draw_tile_cursor(canvas, sv, ct);
        }
    }

    fn draw_except_anim(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &SdlValues,
        player_move_adjust: (i32, i32), player_move_dir: Option<Direction>) {

        canvas.set_viewport(self.rect);
        canvas.set_draw_color(Color::RGB(120, 120, 120));

        let gd = &game.gd;
        let map = gd.get_current_map();
        let view_map = &game.view_map;
        let player_pos = gd.player_pos();
        let prev_player_pos: Option<Vec2d> = if let Some(dir) = player_move_dir {
            Some(player_pos - dir.as_vec())
        } else {
            None
        };
        let prev_player_pos_one_back_side = if let Some(prev_player_pos) = prev_player_pos {
            Some(prev_player_pos + (0, -1))
        } else {
            None
        };
        
        let is_player_moving = player_move_adjust != (0, 0);

        let player_drawing_row = player_pos.1 + if let Some(dir) = player_move_dir {
            match dir.vdir {
                VDirection::Up | VDirection::None => 0,
                VDirection::Down => -1,
            }
        }else{ 0 };
        
        let tile_range = self.tile_range();

        // Draw background parts
        for p in tile_range.clone() {
            self.draw_background_parts(canvas, map, sv, p);
        }

        let player_pos_one_back_side = player_pos + (0, -1);

        // Draw foreground parts
        for ny in tile_range.iter1() {
            for nx in tile_range.iter0() {
                let p = Vec2d::new(nx, ny);

                // Control the order of drawing foreground parts
                // because foreground parts on player's original or destination tiles
                // are drawed before player character drawing.
                // It is needed to make the graphic order more natural
                if !is_player_moving || (p != player_pos && Some(p) != prev_player_pos){
                    self.draw_foreground_parts(
                        canvas, map, view_map, sv, p,
                        gd, is_player_moving, player_move_adjust);
                }
                if is_player_moving && p == player_pos_one_back_side {
                    self.draw_foreground_parts(
                        canvas, map, view_map, sv, player_pos,
                        gd, is_player_moving, player_move_adjust);
                }
                if prev_player_pos_one_back_side == Some(p) {
                    self.draw_foreground_parts(
                        canvas, map, view_map, sv, p + (0, 1),
                        gd, is_player_moving, player_move_adjust);
                }
            }
            // Draw player during moving animation
            if is_player_moving && ny == player_drawing_row {
                let chara = gd.chara.get(CharaId::Player);
                let ct = gobj::get_obj(chara.template);
                let src = Rect::from(ct.img_rect());
                let dest = self.centering_at_tile(src, player_pos,
                                                  -player_move_adjust.0, -player_move_adjust.1);
                canvas.copy(sv.tex().get(chara.template), src, dest).unwrap();
            }
        }
        // Draw background parts
        for p in tile_range {
            self.draw_overlay(canvas, game, sv, p);
        }
    }

    /// Draw tile background parts
    fn draw_background_parts(&self, canvas: &mut WindowCanvas, map: &Map, sv: &SdlValues, p: Vec2d) {
        let di = BackgroundDrawInfo::new(map, p);

        if let Some(t) = di.tile { // Draw tile
            for i in 0..N_TILE_IMG_LAYER {
                if t[i].is_empty() { continue; }
                let obj = gobj::get_obj(t[i].idx);
                let tex = sv.tex().get(t[i].idx);
                self.draw_pieces(canvas, tex, obj, p, t[i].piece_pattern);
            }
        }
        if let Some(special_tile_idx) = di.special { // Draw tile special
            let texture = sv.tex().get(special_tile_idx);
            let query = texture.query();
            let src = Rect::new(0, 0, query.width, query.height);
            let dest = self.bottom_at_tile(src, p, 0, 0);
            check_draw!(canvas.copy(&texture, src, dest));
        }
    }

    /// Draw tile foreground parts
    fn draw_foreground_parts(&self, canvas: &mut WindowCanvas, map: &Map, view_map: &ViewMap,
                             sv: &SdlValues, p: Vec2d,
                             gd: &GameData, is_player_moving: bool, player_move_adjust: (i32, i32)) {
        let di = ForegroundDrawInfo::new(map, view_map, p);

        if let Some(special_tile_idx) = di.special { // Draw tile special
            let tex = sv.tex().get(special_tile_idx);
            let query = tex.query();
            let src = Rect::new(0, 0, query.width, query.height);
            let dest = self.bottom_at_tile(src, p, 0, 0);
            check_draw!(canvas.copy(&tex, src, dest));
        }
        if let Some(wall_idx) = di.wallpp.idx() { // Draw wall
            let obj = gobj::get_obj(wall_idx);
            let tex = sv.tex().get(wall_idx);
            self.draw_pieces(canvas, tex, obj, p, di.wallpp.piece_pattern);
        }

        if let Some(deco_idx) = di.deco { // Draw decoration
            let tex = sv.tex().get(deco_idx);
            let query = tex.query();
            let src = Rect::new(0, 0, query.width, query.height);
            let dest = self.bottom_at_tile(src, p, 0, 0);
            check_draw!(canvas.copy(&tex, src, dest));
        }

        // Draw items
        for i in 0..di.n_item {
            let item_idx = di.items[i];
            let texture = sv.tex().get(item_idx);

            let query = texture.query();
            let src = Rect::new(0, 0, query.width, query.height);
            let dest = self.centering_at_tile(src, p, 0, 0);
        
            check_draw!(canvas.copy(&texture, src, dest));
        }

        // Draw character on the tile
        if let Some(chara_id) = di.chara {
            let chara = gd.chara.get(chara_id);
            let ct = gobj::get_obj(chara.template);
            let src = Rect::from(ct.img_rect());
            
            if !(chara_id == CharaId::Player && is_player_moving) {
                let dest = if chara_id == CharaId::Player {
                    self.centering_at_tile(src, p, -player_move_adjust.0, -player_move_adjust.1)
                }else{
                    self.centering_at_tile(src, p, 0, 0)
                };
                check_draw!(canvas.copy(sv.tex().get(chara.template), src, dest));
            }
        }
    }

    /// Draw overlay for a tile
    fn draw_overlay(&self, canvas: &mut WindowCanvas, game: &Game, sv: &SdlValues, p: Vec2d) {

        match overlay::view_fog(game, p) {
            overlay::FogPattern::None => (),
            overlay::FogPattern::PiecePattern(idx, pp) => {
                let tex = sv.tex().get(idx);
                let obj = gobj::get_obj(idx);
                self.draw_pieces(canvas, tex, obj, p, pp);
            }
            overlay::FogPattern::Fog(idx) => {
                // src rect is fixed at right-bottom corner of image
                let src = Rect::new(TILE_SIZE_I, TILE_SIZE_I * 2, TILE_SIZE, TILE_SIZE);
                let dest = Rect::new(
                    p.0 * TILE_SIZE_I + self.dx, p.1 * TILE_SIZE_I + self.dy,
                    TILE_SIZE, TILE_SIZE);
                let tex = sv.tex().get(idx);
                check_draw!(canvas.copy(&tex, src, dest));
            }
        }
    }

    /// Draw overlay for all tiles
    fn draw_overlay_all(&self, canvas: &mut WindowCanvas, game: &Game, sv: &SdlValues) {
        let idx = if let Some(idx) = overlay::all(game) { idx } else { return; };
        let texture = sv.tex().get(idx);
        let src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
        let (nx, ny) = self.calc_tile_num();

        for iy in 0..ny {
            for ix in 0..nx {
                let dest = Rect::new(
                    ix * TILE_SIZE_I, iy * TILE_SIZE_I,
                    TILE_SIZE, TILE_SIZE);
                check_draw!(canvas.copy(&texture, src, dest));
            }
        }
    }

    fn draw_anim(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &SdlValues,
                 anim: &Animation, i_frame: u32) {
        match anim {
            &Animation::Img { idx, range, .. } => {
                for p in range {
                    let src = Rect::from(gobj::get_obj(idx).img_rect_nth(i_frame));
                    let dest = self.centering_at_tile(src, p, 0, 0);
                    check_draw!(canvas.copy(sv.tex().get(idx), src, dest));
                }
            }
            &Animation::Shot { n_frame, n_image, idx, start, target, dir } => {
                let src = Rect::from(gobj::get_obj(idx).img_rect_nth(n_image));
                let dest = if n_frame - 1 != i_frame {
                    let mut dest = self.centering_at_tile(src, start, 0, 0);
                    dest.x += (dir.0 * (i_frame * TILE_SIZE) as f32) as i32;
                    dest.y += (dir.1 * (i_frame * TILE_SIZE) as f32) as i32;
                    dest
                } else {
                    self.centering_at_tile(src, target, 0, 0)
                };
                check_draw!(canvas.copy(sv.tex().get(idx), src, dest));
            }
            _ => (),
        }
    }

    fn draw_pieces<T: PieceImgObject>(
        &self, canvas: &mut WindowCanvas, tex: &Texture, obj: &T,
        p: Vec2d, piece_pattern: PiecePattern) {
        
        let img = obj.get_img();
        let i_anim_frame = calc_frame(img);
        let dy = TILE_SIZE_I - img.h as i32;
        // Top left corner (x ,y)
        let tlcx = TILE_SIZE_I * p.0 + self.dx;
        let tlcy = TILE_SIZE_I * p.1 + self.dy + dy;
        
        // Top left piece
        if let Some(src) = obj.piece_rect(piece_pattern.top_left, 0, i_anim_frame) {
            let dest = Rect::new(tlcx, tlcy, src.2, src.3);
            let src = Rect::from(src);
            check_draw!(canvas.copy(tex, src, dest));
        }
        // Top right piece
        if let Some(src) = obj.piece_rect(piece_pattern.top_right, 1, i_anim_frame) {
            let dest = Rect::new(tlcx + PIECE_SIZE_I, tlcy, src.2, src.3);
            let src = Rect::from(src);
            check_draw!(canvas.copy(tex, src, dest));
        }
        // Bottom left piece
        if let Some(src) = obj.piece_rect(piece_pattern.bottom_left, 2, i_anim_frame) {
            let dest = Rect::new(tlcx, tlcy + PIECE_SIZE_I, src.2, src.3);
            let src = Rect::from(src);
            check_draw!(canvas.copy(tex, src, dest));
        }
        // Bottom right piece
        if let Some(src) = obj.piece_rect(piece_pattern.bottom_right, 3, i_anim_frame) {
            let dest = Rect::new(tlcx + PIECE_SIZE_I, tlcy + PIECE_SIZE_I, src.2, src.3);
            let src = Rect::from(src);
            check_draw!(canvas.copy(tex, src, dest));
        }
    }

    fn draw_tile_cursor(&self, canvas: &mut WindowCanvas, sv: &SdlValues, ct: Vec2d) {
        let idx: UIImgIdx = gobj::id_to_idx_checked("!tile-cursor")
            .expect("UIImg object \"!tile-cursor\" not found");
        
        let src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
        let dest = self.centering_at_tile(src, ct, 0, 0);
        check_draw!(canvas.copy(sv.tex().get(idx), src, dest));
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
        let bottom_right = Vec2d::new(nx + top_left.0, ny + top_left.1 + 1);
        RectIter::new(top_left, bottom_right)
    }
}

