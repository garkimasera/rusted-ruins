use super::commonuse::*;
use crate::config::SCREEN_CFG;
use crate::config::UI_CFG;
use crate::game::command::MouseButton;
use common::gobj;
use common::objholder::UIImgIdx;

pub struct Sidebar {
    rect: Rect,
    mouseover: Option<u32>,
}

lazy_static! {
    static ref ICON_IDX: UIImgIdx = gobj::id_to_idx("sidebar-icon");
}

const ITEM_INVENTORY: u32 = 0;
const ITEM_EQUIPMENT: u32 = 1;
const ITEM_CHARAINFO: u32 = 2;
const ITEM_CREATION: u32 = 3;
const ITEM_GAMEINFO: u32 = 4;
const ITEM_SAVE: u32 = 5;
const N_ITEM: u32 = 6;

impl Sidebar {
    pub fn new() -> Sidebar {
        let pos = SCREEN_CFG.sidebar;
        let cfg = &UI_CFG.sidebar;
        let rect = Rect::new(pos.x, pos.y, cfg.icon_w, (cfg.icon_h + cfg.space) * N_ITEM);

        Sidebar {
            rect,
            mouseover: None,
        }
    }
}

impl Window for Sidebar {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        let cfg = &UI_CFG.sidebar;
        context.set_viewport(None);

        context.fill_rect(SCREEN_CFG.sidebar, UI_CFG.color.sidebar_bg);

        for i in 0..N_ITEM {
            let rect = Rect::new(
                self.rect.x,
                self.rect.y + (cfg.icon_h + cfg.space) as i32 * i as i32,
                cfg.icon_w,
                cfg.icon_h,
            );
            let mouseover = if let Some(mouseover) = self.mouseover.as_ref() {
                if *mouseover == i {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            context.render_tex_n(*ICON_IDX, rect, i * 2 + mouseover);
        }
    }
}

impl DialogWindow for Sidebar {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::MouseState { x, y, .. } => {
                self.mouseover = None;
                if self.rect.contains_point((*x, *y)) {
                    let i =
                        (*y - self.rect.y) as u32 / (UI_CFG.sidebar.icon_h + UI_CFG.sidebar.space);
                    if i < N_ITEM {
                        self.mouseover = Some(i);
                    }
                    return DialogResult::Command(None);
                }
            }
            Command::MouseButtonDown { x, y, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                return DialogResult::Command(None);
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                if *button != MouseButton::Left {
                    return DialogResult::Command(None);
                }
                let i = (*y - self.rect.y) as u32 / (UI_CFG.sidebar.icon_h + UI_CFG.sidebar.space);
                if i == ITEM_INVENTORY {
                    return DialogResult::Command(Some(Command::OpenItemMenu));
                } else if i == ITEM_EQUIPMENT {
                    return DialogResult::Command(Some(Command::OpenEquipWin));
                } else if i == ITEM_CHARAINFO {
                    return DialogResult::Command(Some(Command::OpenStatusWin));
                } else if i == ITEM_CREATION {
                    return DialogResult::Command(Some(Command::OpenCreationWin));
                } else if i == ITEM_GAMEINFO {
                    return DialogResult::Command(Some(Command::OpenGameInfoWin));
                } else if i == ITEM_SAVE {
                    return DialogResult::Command(Some(Command::OpenExitWin));
                }
            }
            _ => (),
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
