use super::widget::*;
use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_rect_border;
use crate::eventhandler::InputMode;
use crate::game::item::info::ItemInfoText;
use crate::game::{Animation, Command, DoPlayerAction, Game};
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::ItemLocation;
use sdl2::rect::Rect;

pub struct ItemInfoWindow {
    rect: Rect,
    il: ItemLocation,
    item_image: ImageWidget,
    item_name: LabelWidget,
    item_kind: LabelWidget,
    basic_desc: LabelWidget,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation, game: &Game) -> ItemInfoWindow {
        let c = &UI_CFG.item_info_window;
        let item = game.gd.get_item(il);
        let info = ItemInfoText::new(item.0);

        let item_image = ImageWidget::item(c.item_image, game.gd.get_item(il).0.idx);
        let item_name = LabelWidget::new(c.item_name, &info.item_name, FontKind::M);
        let item_kind = LabelWidget::new(c.item_kind, &info.item_kind, FontKind::M);
        let basic_desc = LabelWidget::new(c.basic_desc, &info.basic_desc, FontKind::M);

        ItemInfoWindow {
            rect: UI_CFG.item_info_window.rect.into(),
            il,
            item_image,
            item_name,
            item_kind,
            basic_desc,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);

        self.item_image.draw(context);
        self.item_name.draw(context);
        self.item_kind.draw(context);
        self.basic_desc.draw(context);
    }
}

impl DialogWindow for ItemInfoWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
