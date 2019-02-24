use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_rect_border;
use crate::eventhandler::InputMode;
use crate::game::{Animation, Command, DoPlayerAction, Game, InfoGetter};
use crate::window::{DialogResult, DialogWindow, Window, WindowDrawMode};
use common::gamedata::ItemLocation;
use sdl2::rect::Rect;

pub struct ItemInfoWindow {
    rect: Rect,
    il: ItemLocation,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation) -> ItemInfoWindow {
        ItemInfoWindow {
            rect: UI_CFG.item_info_window.rect.into(),
            il,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);
    }
}

impl DialogWindow for ItemInfoWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
