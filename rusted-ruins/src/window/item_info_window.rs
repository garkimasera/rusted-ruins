use super::widget::*;
use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_window_border;
use crate::game::item::info::ItemInfoText;
use crate::game::{Animation, Command, DoPlayerAction, Game};
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::ItemLocation;
use common::gobj;
use common::objholder::UiImgIdx;
use sdl2::rect::Rect;

pub struct ItemInfoWindow {
    rect: Rect,
    item_image: ImageWidget,
    item_name: LabelWidget,
    item_kind: LabelWidget,
    flavor_text: LabelWidget,
    attrs_list: ListWidget<(IconIdx, TextCache)>,
    escape_click: bool,
}

impl ItemInfoWindow {
    pub fn new(il: ItemLocation, game: &Game) -> ItemInfoWindow {
        let c = &UI_CFG.item_info_window;
        let rect: Rect = c.rect.into();
        let item = game.gd.get_item(il);
        let info = ItemInfoText::new(item.0);

        let item_image = ImageWidget::item(c.item_image, game.gd.get_item(il).0);
        let item_name = LabelWidget::new(c.item_name, &info.item_name, FontKind::M);
        let item_kind = LabelWidget::new(c.item_kind, &info.item_kind, FontKind::S);

        let flavor_text = LabelWidget::wrapped(
            Rect::new(0, c.flavor_text_y, 0, 0),
            crate::text::flavor_txt_checked(gobj::idx_to_id(item.0.idx))
                .unwrap_or_else(|| "".into()),
            FontKind::S,
            rect.width(),
        );

        let items = info
            .desc_text
            .iter()
            .map(|(img_id, t)| {
                let img_idx: UiImgIdx = gobj::id_to_idx(img_id);
                let t = TextCache::new(t, FontKind::M, UI_CFG.color.normal_font);
                (IconIdx::from(img_idx), t)
            })
            .collect();
        let list_rect = Rect::new(0, c.list_y, rect.width(), rect.height() - c.list_y as u32);
        let mut attrs_list = ListWidget::with_scroll_bar(
            list_rect,
            c.column_pos.clone(),
            list_rect.height() / UI_CFG.list_widget.h_row_default,
            false,
        );
        attrs_list.no_row_highlight();
        attrs_list.set_items(items);

        ItemInfoWindow {
            rect: UI_CFG.item_info_window.rect.into(),
            item_image,
            item_name,
            item_kind,
            flavor_text,
            attrs_list,
            escape_click: false,
        }
    }
}

impl Window for ItemInfoWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        let line_y = UI_CFG.item_info_window.list_y - 1;
        context.draw_line((0, line_y), (self.rect.w, line_y), UI_CFG.color.border_dark);

        self.item_image.draw(context);
        self.item_name.draw(context);
        self.item_kind.draw(context);
        self.flavor_text.draw(context);
        self.attrs_list.draw(context);
    }
}

impl DialogWindow for ItemInfoWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        check_escape_click!(self, command, false);

        let command = command.relative_to(self.rect);

        self.attrs_list.process_command(&command);

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}
