use super::commonuse::*;
use super::widget::*;
use crate::game::extrait::ItemExt;
use crate::text::ToText;
use common::gamedata::*;

pub struct SlotInstallWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, TextCache, TextCache)>,
    kind: ModuleSlotKind,
    escape_click: bool,
}

impl SlotInstallWindow {
    pub fn new(gd: &GameData, kind: ModuleSlotKind) -> SlotInstallWindow {
        let cfg = &UI_CFG.slot_window;
        let rect: Rect = cfg.rect.into();
        let list = ListWidget::new(
            Rect::new(0, 0, rect.width(), rect.height()),
            cfg.column_pos.clone(),
            cfg.n_row,
            false,
        );

        let mut window = SlotInstallWindow {
            rect,
            list,
            kind,
            escape_click: false,
        };
        window.update(gd);
        window
    }

    pub fn update(&mut self, gd: &GameData) {
        let rows: Vec<_> = crate::game::item::slot::slot_installable_item_list(gd, self.kind)
            .iter()
            .map(|(il, cost)| {
                let item = &gd.get_item(*il).0;
                (
                    item.icon(),
                    TextCache::new(item.to_text(), FontKind::M, UI_CFG.color.normal_font),
                    TextCache::new(format!("{}", cost), FontKind::M, UI_CFG.color.normal_font),
                )
            })
            .collect();
        self.list.set_items(rows);
    }
}

impl Window for SlotInstallWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for SlotInstallWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        check_escape_click!(self, command, false);
        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(_i)) = self.list.process_command(&command) {
            // Any item is selected
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }
}
