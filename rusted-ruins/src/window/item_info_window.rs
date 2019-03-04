use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_rect_border;
use crate::eventhandler::InputMode;
use crate::game::{Animation, Command, DoPlayerAction, Game};
use crate::game::item::info::ItemInfoText;
use crate::window::{DialogResult, DialogWindow, Window, WindowDrawMode};
use super::widget::*;
use common::gamedata::ItemLocation;
use sdl2::rect::Rect;

pub struct ItemInfoWindow {
    rect: Rect,
    il: ItemLocation,
    item_kind: LabelWidget,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation, game: &Game) -> ItemInfoWindow {
        let c = &UI_CFG.item_info_window;
        let item = game.gd.get_item(il);
        let info = ItemInfoText::new(item.0);

        let item_kind = LabelWidget::new(c.item_kind, &info.item_kind, FontKind::M);
        
        ItemInfoWindow {
            rect: UI_CFG.item_info_window.rect.into(),
            il,
            item_kind,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);

        self.item_kind.draw(context);
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
