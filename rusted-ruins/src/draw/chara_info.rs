use super::mainwin::MainWinDrawer;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::Game;
use common::gobj;
use common::objholder::UIImgIdx;
use once_cell::sync::Lazy;
use sdl2::rect::Rect;

struct DrawInfo {
    target_icon_idx: UIImgIdx,
    target_icon_x: i32,
    target_icon_y: i32,
    target_icon_w: u32,
    target_icon_h: u32,
}

impl DrawInfo {
    fn init() -> DrawInfo {
        let target_icon_idx: UIImgIdx = gobj::id_to_idx("!target-icon");
        let obj = gobj::get_obj(target_icon_idx);
        DrawInfo {
            target_icon_idx,
            target_icon_x: UI_CFG.chara_info.target_icon_x,
            target_icon_y: UI_CFG.chara_info.target_icon_y,
            target_icon_w: obj.img.w,
            target_icon_h: obj.img.h,
        }
    }
}

static DRAW_INFO: Lazy<DrawInfo> = Lazy::new(|| DrawInfo::init());

impl MainWinDrawer {
    pub fn draw_chara_info(&self, context: &mut Context, game: &Game) {
        let map = game.gd.get_current_map();
        let target_chara = game.target_chara();

        for p in self.tile_range() {
            if !map.is_inside(p) {
                continue;
            }
            let tile_info = &map.tile[p];
            let cid = if let Some(cid) = tile_info.chara.as_ref() {
                *cid
            } else {
                continue;
            };

            if !game.view_map.get_tile_visible(p) {
                continue;
            }

            let tile_rect = self.tile_rect(p, 0, 0);
            // Draw target icon
            if target_chara.is_some() && target_chara.unwrap() == cid {
                context.render_tex(
                    DRAW_INFO.target_icon_idx,
                    Rect::new(
                        tile_rect.x + DRAW_INFO.target_icon_x,
                        tile_rect.y + DRAW_INFO.target_icon_y,
                        DRAW_INFO.target_icon_w,
                        DRAW_INFO.target_icon_h,
                    ),
                );
            }
        }
    }
}
