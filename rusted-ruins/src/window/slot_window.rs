use super::commonuse::*;
use super::item_window::{ActionCallback, ItemWindow, ItemWindowMode};
use super::widget::*;
use crate::game::extrait::ItemExt;
use crate::game::item::filter::ItemFilter;
use crate::text::ToText;
use common::gamedata::*;
use common::gobj;
use common::objholder::UiImgIdx;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SlotInstallWindow {
    rect: Rect,
    closer: DialogCloser,
    il_cost: Vec<(ItemLocation, i64)>,
    list: ListWidget<(IconIdx, TextCache, TextCache)>,
    kind: ModuleSlotKind,
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
            closer: DialogCloser::new(rect),
            il_cost: Vec::new(),
            list,
            kind,
        };
        window.update(gd);
        window
    }

    pub fn update(&mut self, gd: &GameData) {
        self.il_cost = crate::game::item::slot::slot_installable_item_list(gd, self.kind);
        let rows: Vec<_> = self
            .il_cost
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
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for SlotInstallWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        closer!(self, command, false);
        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            // Any item is selected
            pa.install_slot(
                self.il_cost[i as usize].0,
                self.kind,
                self.il_cost[i as usize].1,
            );
            self.update(pa.gd());
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

pub fn slot_insertable_item_window(game: &Game) -> ItemWindow {
    let action: Box<ActionCallback> = Box::new(|pa, il| {
        let item = pa.remove_item(il, 1);
        let win = SlotInsertWindow::new(item);
        DialogResult::OpenChildDialog(Box::new(win))
    });
    let mode = ItemWindowMode::Select {
        ill: ItemListLocation::PLAYER,
        filter: ItemFilter::default().module_insertable(true),
        action,
    };
    ItemWindow::new(mode, game)
}

pub struct SlotInsertWindow {
    rect: Rect,
    closer: DialogCloser,
    slots: Vec<(ModuleSlotKind, String)>,
    list: ListWidget<(IconIdx, TextCache)>,
    item: Rc<RefCell<Item>>,
}

impl SlotInsertWindow {
    pub fn new(item: Item) -> SlotInsertWindow {
        let cfg = &UI_CFG.slot_window;
        let rect: Rect = cfg.rect.into();
        let list = ListWidget::new(
            Rect::new(0, 0, rect.width(), rect.height()),
            cfg.column_pos.clone(),
            cfg.n_row,
            false,
        );

        let mut window = SlotInsertWindow {
            rect,
            closer: DialogCloser::new(rect),
            slots: Vec::new(),
            list,
            item: Rc::new(RefCell::new(item)),
        };
        window.update();
        window
    }

    pub fn update(&mut self) {
        self.slots = crate::game::item::slot::slot_list(&*self.item.borrow());
        let rows: Vec<_> = self
            .slots
            .iter()
            .map(|(kind, text)| {
                let img_idx: UiImgIdx = gobj::id_to_idx(crate::game::item::info::slot_icon(*kind));
                (
                    IconIdx::from(img_idx),
                    TextCache::new(text, FontKind::M, UI_CFG.color.normal_font),
                )
            })
            .collect();
        self.list.set_items(rows);
    }

    fn process_command_inner(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_>,
    ) -> DialogResult {
        closer!(self, command, false);
        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            let win = module_select_item_window(
                pa.game(),
                self.item.clone(),
                self.slots[i as usize].0,
                i,
            );
            return DialogResult::OpenChildDialog(Box::new(win));
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}

impl Window for SlotInsertWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for SlotInsertWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        let result = self.process_command_inner(command, pa);
        if result.is_close() {
            pa.append_item(self.item.borrow().clone(), 1);
        }
        result
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }

    fn callback_child_closed(
        &mut self,
        _result: Option<DialogCloseValue>,
        _pa: &mut DoPlayerAction<'_>,
    ) -> DialogResult {
        self.update();
        DialogResult::Continue
    }
}

pub fn module_select_item_window(
    game: &Game,
    item: Rc<RefCell<Item>>,
    kind: ModuleSlotKind,
    i_slot: u32,
) -> ItemWindow {
    let action: Box<ActionCallback> = Box::new(move |pa, il| {
        let module_item = pa.remove_item(il, 1);
        if let Some(removed_module_item) =
            crate::game::item::slot::insert_module_to(&mut *item.borrow_mut(), module_item, i_slot)
        {
            pa.append_item(removed_module_item, 1);
        }
        DialogResult::Close
    });
    let mode = ItemWindowMode::Select {
        ill: ItemListLocation::PLAYER,
        filter: ItemFilter::default().module_kind(kind),
        action,
    };
    ItemWindow::new(mode, game)
}
