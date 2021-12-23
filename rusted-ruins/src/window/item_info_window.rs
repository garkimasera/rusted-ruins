use super::widget::*;
use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_window_border;
use crate::game::item::info::ItemInfoText;
use crate::game::{Animation, Command, DoPlayerAction, Game};
use crate::window::flavor_text_window::FlavorTextWindow;
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::ItemLocation;
use common::gobj;
use sdl2::rect::Rect;

pub struct ItemInfoWindow {
    rect: Rect,
    il: ItemLocation,
    item_image: ImageWidget,
    item_name: LabelWidget,
    item_kind: LabelWidget,
    desc_text: Vec<(ImageWidget, LabelWidget)>,
    flavor_button: Option<ButtonWidget>,
    flavor_text: Option<FlavorTextWindow>,
    size_adjusted: bool,
    escape_click: bool,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation, game: &Game) -> ItemInfoWindow {
        let c = &UI_CFG.item_info_window;
        let item = game.gd.get_item(il);
        let info = ItemInfoText::new(item.0);

        let item_image = ImageWidget::item(c.item_image, game.gd.get_item(il).0);
        let item_name = LabelWidget::new(c.item_name, &info.item_name, FontKind::M);
        let item_kind = LabelWidget::new(c.item_kind, &info.item_kind, FontKind::M);

        let flavor_button = crate::text::flavor_txt_checked(gobj::idx_to_id(item.0.idx))
            .map(|_| ButtonWidget::new(c.flavor_button, "i", FontKind::T));

        let mut desc_text = Vec::new();
        let rect = Rect::new(0, 0, 1, 1);
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
            flavor_button,
            flavor_text: None,
            desc_text,
            size_adjusted: false,
            escape_click: false,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);

        let c = &UI_CFG.item_info_window;
        let label_rect: Rect = c.desc_text.into();
        let icon_rect: Rect = c.desc_text_icon.into();
        let mut label_y = label_rect.y;
        let mut icon_y = icon_rect.y;

        if let Some(flavor_button) = self.flavor_button.as_mut() {
            flavor_button.draw(context);
        }

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

        if let Some(flavor_text) = self.flavor_text.as_mut() {
            flavor_text.draw(context, game, anim);
        }
    }
}

impl DialogWindow for ItemInfoWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if let Some(flavor_text) = self.flavor_text.as_mut() {
            match flavor_text.process_command(command, pa) {
                DialogResult::Continue => {
                    return DialogResult::Continue;
                }
                DialogResult::Close => {
                    self.flavor_text = None;
                    return DialogResult::Continue;
                }
                _ => unreachable!(),
            }
        }

        check_escape_click!(self, command, false);

        let command = command.relative_to(self.rect);

        if let Some(flavor_button) = self.flavor_button.as_mut() {
            if flavor_button.process_command(&command).is_some() {
                let item = pa.gd().get_item(self.il).0;
                let id = gobj::idx_to_id(item.idx);
                self.flavor_text = FlavorTextWindow::new(id, ImageIdx::item(item));
            }
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}
