use super::mainwin::MainWinDrawer;
use crate::config::UI_CFG;
use crate::context::*;
use crate::damage_popup::{DamagePopup, PopupKind};
use common::gobj;
use common::objholder::UiImgIdx;
use geom::Coords;
use once_cell::sync::Lazy;
use sdl2::rect::Rect;

struct DigitDrawInfo {
    idx: UiImgIdx,
    digit_w: i32,
    digit_h: i32,
}

impl DigitDrawInfo {
    fn init() -> DigitDrawInfo {
        let idx: UiImgIdx = gobj::id_to_idx("!numbers-damage");
        let obj = gobj::get_obj(idx);
        DigitDrawInfo {
            idx,
            digit_w: obj.img.w as i32,
            digit_h: obj.img.h as i32,
        }
    }
}

static DIGIT_DRAW_INFO: Lazy<DigitDrawInfo> = Lazy::new(DigitDrawInfo::init);
static POPUP_MISS_IDX: Lazy<UiImgIdx> = Lazy::new(|| gobj::id_to_idx("!popup-miss"));

impl MainWinDrawer {
    pub fn draw_damage(&self, context: &mut Context<'_, '_, '_, '_>) {
        let damage_popup_list = &crate::damage_popup::get();
        for popup in &damage_popup_list.popup_list {
            self.draw_damage_to_tile(context, popup);
        }
    }

    fn draw_damage_to_tile(&self, context: &mut Context<'_, '_, '_, '_>, popup: &DamagePopup) {
        let passed_frame_min = popup.queue[0].0;

        for (i, popup_kind) in popup.queue.iter().enumerate() {
            match popup_kind.1 {
                PopupKind::Damage(value) => {
                    self.draw_digits_to_tile(
                        context,
                        value,
                        0,
                        popup.pos,
                        calc_dy(passed_frame_min, i),
                    );
                }
                PopupKind::Heal(value) => {
                    self.draw_digits_to_tile(
                        context,
                        value,
                        1,
                        popup.pos,
                        calc_dy(passed_frame_min, i),
                    );
                }
                PopupKind::Miss => {
                    self.draw_popup_img_to_tile(
                        context,
                        *POPUP_MISS_IDX,
                        popup.pos,
                        calc_dy(passed_frame_min, i),
                    );
                }
            }
        }
    }

    fn draw_digits_to_tile(
        &self,
        context: &mut Context<'_, '_, '_, '_>,
        value: i32,
        color: u32,
        pos: Coords,
        dy: i32,
    ) {
        let digit_w = DIGIT_DRAW_INFO.digit_w;
        let digit_h = DIGIT_DRAW_INFO.digit_h;

        let s = format!("{value}");
        let string_w = s.len() as i32 * digit_w;
        let tile_rect = self.tile_rect(pos, 0, 0);
        let top_left_x = tile_rect.x + (tile_rect.w - string_w) / 2;
        let top_left_y = tile_rect.y + dy;

        for (i, n) in s.chars().enumerate() {
            let i_img = n as u32 - '0' as u32;
            let rect = Rect::new(
                top_left_x + i as i32 * digit_w,
                top_left_y,
                digit_w as u32,
                digit_h as u32,
            );
            context.render_tex_n(DIGIT_DRAW_INFO.idx, rect, i_img + 10 * color);
        }
    }

    fn draw_popup_img_to_tile(
        &self,
        context: &mut Context<'_, '_, '_, '_>,
        idx: UiImgIdx,
        pos: Coords,
        dy: i32,
    ) {
        let obj = gobj::get_obj(idx);
        let tile_rect = self.tile_rect(pos, 0, 0);
        let top_left_x = tile_rect.x + (tile_rect.w - obj.img.w as i32) / 2;
        let top_left_y = tile_rect.y + dy;
        let rect = Rect::new(top_left_x, top_left_y, obj.img.w, obj.img.h);
        context.render_tex_n(idx, rect, 0);
    }
}

fn calc_dy(passed_frame: u32, n: usize) -> i32 {
    UI_CFG.damage_popup.start_dy
        - passed_frame as i32 * UI_CFG.damage_popup.speed
        - n as i32 * UI_CFG.damage_popup.digit_h
}
