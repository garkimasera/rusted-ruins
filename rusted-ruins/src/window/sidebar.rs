use super::commonuse::*;
use crate::config::SCREEN_CFG;
use crate::config::UI_CFG;
use common::gobj;
use common::objholder::UIImgIdx;

pub struct Sidebar {
    rect: Rect,
}

const ICON_ID: &[&str] = &["sidebar-inventory", "sidebar-equip"];

lazy_static! {
    static ref ICON_IDX: Vec<UIImgIdx> = ICON_ID.iter().map(|id| gobj::id_to_idx(id)).collect();
}

impl Sidebar {
    pub fn new() -> Sidebar {
        let pos = SCREEN_CFG.sidebar;
        let cfg = &UI_CFG.sidebar;
        let n_item = ICON_ID.len();
        let rect = Rect::new(
            pos.x,
            pos.y,
            cfg.icon_w,
            (cfg.icon_h + cfg.space) * n_item as u32,
        );

        Sidebar { rect }
    }
}

impl Window for Sidebar {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        let cfg = &UI_CFG.sidebar;
        context.set_viewport(None);

        for (i, &icon_idx) in ICON_IDX.iter().enumerate() {
            let rect = Rect::new(
                self.rect.x,
                self.rect.y + (cfg.icon_h + cfg.space) as i32 * i as i32,
                cfg.icon_w,
                cfg.icon_h,
            );
            dbg!(rect);
            context.render_tex(icon_idx, rect);
        }
    }
}
