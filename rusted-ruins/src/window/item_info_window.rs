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
    desc_text: Vec<(ImageWidget, LabelWidget)>,
    size_adjusted: bool,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation, game: &Game) -> ItemInfoWindow {
        let c = &UI_CFG.item_info_window;
        let item = game.gd.get_item(il);
        let info = ItemInfoText::new(item.0);

        let item_image = ImageWidget::item(c.item_image, game.gd.get_item(il).0.idx);
        let item_name = LabelWidget::new(c.item_name, &info.item_name, FontKind::M);
        let item_kind = LabelWidget::new(c.item_kind, &info.item_kind, FontKind::M);

        let mut desc_text = Vec::new();
        let mut rect = Rect::new(0, 0, 1, 1);
        for (img_id, t) in &info.desc_text {
            let w0 = ImageWidget::ui_img(rect, img_id);
            let w1 = LabelWidget::wrapped(
                rect,
                t,
                FontKind::M,
                Into::<Rect>::into(c.desc_text).width(),
            );
            desc_text.push((w0, w1));
        }

        ItemInfoWindow {
            rect: UI_CFG.item_info_window.rect.into(),
            il,
            item_image,
            item_name,
            item_kind,
            desc_text,
            size_adjusted: false,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);

        let c = &UI_CFG.item_info_window;
        let label_rect: Rect = c.desc_text.into();
        let icon_rect: Rect = c.desc_text_icon.into();
        let mut label_y = label_rect.y;
        let mut icon_y = icon_rect.y;

        if !self.size_adjusted {
            for (icon, label) in &mut self.desc_text {
                let size = label.adjust_widget_size(context.sv);
                icon.set_rect(Rect::new(
                    icon_rect.x,
                    icon_y,
                    icon_rect.width(),
                    icon_rect.height(),
                ));
                label.set_rect(Rect::new(label_rect.x, label_y, size.0, size.1));
                label_y += size.1 as i32;
                icon_y += size.1 as i32;
            }
            self.size_adjusted = true;
        }

        self.item_image.draw(context);
        self.item_name.draw(context);
        self.item_kind.draw(context);

        for (icon, label) in &mut self.desc_text {
            icon.draw(context);
            label.draw(context);
        }
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
