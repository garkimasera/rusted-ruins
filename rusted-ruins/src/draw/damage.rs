use super::mainwin::MainWinDrawer;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::damage::DamagedChara;
use crate::game::Game;
use common::gobj;
use common::objholder::UIImgIdx;
use sdl2::rect::Rect;

struct DigitDrawInfo {
    idx: UIImgIdx,
    digit_w: i32,
    digit_h: i32,
}

impl DigitDrawInfo {
    fn init() -> DigitDrawInfo {
        let idx: UIImgIdx = gobj::id_to_idx("!numbers-damage");
        let obj = gobj::get_obj(idx);
        DigitDrawInfo {
            idx,
            digit_w: obj.img.w as i32,
            digit_h: obj.img.h as i32,
        }
    }
}

lazy_static! {
    static ref DIGIT_DRAW_INFO: DigitDrawInfo = DigitDrawInfo::init();
}

impl MainWinDrawer {
    pub fn draw_damage(&self, context: &mut Context, game: &Game) {
        for damaged_chara in game.damage_view.iter() {
            self.draw_damage_to_tile(context, damaged_chara);
        }
    }

    fn draw_damage_to_tile(&self, context: &mut Context, damaged_chara: &DamagedChara) {
        let digit_w = DIGIT_DRAW_INFO.digit_w;
        let digit_h = DIGIT_DRAW_INFO.digit_h;
        let s = format!("{}", damaged_chara.damage);
        let string_w = s.len() as i32 * digit_w;
        let tile_rect = self.tile_rect(damaged_chara.pos, 0, 0);
        let top_left_x = tile_rect.x + (tile_rect.w - string_w) / 2;
        let top_left_y =
            tile_rect.y + UI_CFG.damage.damage_view_dy - damaged_chara.passed_frame as i32 * 2;

        for (i, n) in s.chars().enumerate() {
            let i_img = n as u32 - '0' as u32;
            let rect = Rect::new(
                top_left_x + i as i32 * digit_w,
                top_left_y,
                digit_w as u32,
                digit_h as u32,
            );
            context.render_tex_n(DIGIT_DRAW_INFO.idx, rect, i_img);
        }
    }
}
