use super::group_window::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_window_border;
use crate::eventhandler::InputMode;
use crate::game::extrait::*;
use crate::game::item::filter::*;
use crate::game::{Animation, Command, DialogOpenRequest, DoPlayerAction, Game, InfoGetter};
use crate::text::ToText;
use crate::window::{DialogResult, DialogWindow, Window, WindowDrawMode};
use common::gamedata::*;
use common::gobj;
use sdl2::rect::Rect;

pub type ActionCallback = dyn FnMut(&mut DoPlayerAction, ItemLocation) -> DialogResult;
pub enum ItemWindowMode {
    List,
    PickUp,
    Drop,
    Drink,
    Eat,
    Use,
    Release,
    ShopSell,
    ShopBuy {
        cid: CharaId,
    },
    Select {
        ill: ItemListLocation,
        filter: ItemFilter,
        action: Box<ActionCallback>,
    },
}

pub struct ItemWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, TextCache, LabelWidget)>,
    mode: ItemWindowMode,
    item_locations: Vec<ItemLocation>,
    escape_click: bool,
    info_label: LabelWidget,
}

const STATUS_WINDOW_GROUP_SIZE: u32 = 6;

pub fn create_item_window_group(game: &Game, mode: ItemWindowMode) -> GroupWindow {
    let mem_info = vec![
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-list"),
            text_id: "tab_text-item_list",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::List, game)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-drop"),
            text_id: "tab_text-item_drop",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::Drop, game)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-drink"),
            text_id: "tab_text-item_drink",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::Drink, game)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-eat"),
            text_id: "tab_text-item_eat",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::Eat, game)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-use"),
            text_id: "tab_text-item_use",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::Use, game)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-item-release"),
            text_id: "tab_text-item_release",
            creator: |game| Box::new(ItemWindow::new(ItemWindowMode::Release, game)),
        },
    ];
    let rect: Rect = UI_CFG.item_window.rect.into();
    let i = match mode {
        ItemWindowMode::List => 0,
        ItemWindowMode::Drop => 1,
        ItemWindowMode::Drink => 2,
        ItemWindowMode::Eat => 3,
        ItemWindowMode::Release => 4,
        _ => unreachable!(),
    };

    GroupWindow::new(
        STATUS_WINDOW_GROUP_SIZE,
        i,
        game,
        mem_info,
        (rect.x, rect.y),
    )
}

impl ItemWindow {
    pub fn new(mode: ItemWindowMode, game: &Game) -> ItemWindow {
        let rect = UI_CFG.item_window.rect.into();
        let n_row = UI_CFG.item_window.n_row;
        let list_h = UI_CFG.list_widget.h_row_default;

        let mut item_window = ItemWindow {
            rect,
            list: ListWidget::with_scroll_bar(
                (0i32, 0i32, rect.w as u32, n_row * list_h),
                UI_CFG.item_window.column_pos.clone(),
                n_row,
                true,
            ),
            mode,
            item_locations: Vec::new(),
            escape_click: false,
            info_label: LabelWidget::new(UI_CFG.item_window.info_label_rect, "", FontKind::M),
        };
        item_window.update_by_mode(&game.gd);
        item_window
    }

    pub fn new_select(
        ill: ItemListLocation,
        filter: ItemFilter,
        action: Box<ActionCallback>,
        pa: &mut DoPlayerAction,
    ) -> ItemWindow {
        let mode = ItemWindowMode::Select {
            ill,
            filter,
            action,
        };
        ItemWindow::new(mode, pa.game())
    }

    fn update_by_mode(&mut self, gd: &GameData) {
        let ill_player = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
        let ill_ground = ItemListLocation::OnMap {
            mid: gd.get_current_mapid(),
            pos: gd.player_pos(),
        };

        match self.mode {
            ItemWindowMode::List => {
                let filtered_list = gd.get_filtered_item_list(ill_player, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::PickUp => {
                let filtered_list = gd.get_filtered_item_list(ill_ground, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Drop => {
                let filtered_list = gd.get_filtered_item_list(ill_player, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Drink => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().drinkable(true),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::Eat => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().eatable(true),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::Use => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().usable(true),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::Release => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().kind_rough(ItemKindRough::MagicDevice),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::ShopBuy { cid } => {
                let ill = ItemListLocation::Shop { cid };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::new());
                self.update_list(filtered_list);
            }
            ItemWindowMode::ShopSell => {
                let ill = ItemListLocation::Chara {
                    cid: CharaId::Player,
                };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::new());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Select { ill, filter, .. } => {
                let filtered_list = gd.get_filtered_item_list(ill, filter);
                self.update_list(filtered_list);
            }
        }
        self.update_label(gd);
    }

    fn update_list(&mut self, list: FilteredItemList) {
        self.list.set_n_item(list.clone().count() as u32);

        let mode = &self.mode;

        self.item_locations.clear();
        for (il, _, _) in list.clone() {
            self.item_locations.push(il);
        }

        let window_width = self.rect.width();

        self.list.update_rows_by_func(move |i| {
            let (_, ref item, n_item) = list.clone().nth(i as usize).unwrap();

            let item_text = format!("{} x {}", item.to_text(), n_item);

            // Infomation displayed in the right column
            let additional_info = match mode {
                ItemWindowMode::ShopBuy { .. } => format!("{}G", item.price()),
                ItemWindowMode::ShopSell => format!("{}G", item.selling_price()),
                _ => format!("{:.2}kg", item.w() as f32 / 1000.0),
            };

            let t1 = TextCache::one(item_text, FontKind::M, UI_CFG.color.normal_font.into());
            let w = window_width
                - UI_CFG.item_window.column_pos.clone()[2] as u32
                - UI_CFG.vscroll_widget.width;
            let t2 = LabelWidget::new(
                Rect::new(0, 0, w, UI_CFG.list_widget.h_row_default),
                &additional_info,
                FontKind::M,
            )
            .right();

            (IconIdx::Item(item.idx), t1, t2)
        });
    }

    fn update_label(&mut self, gd: &GameData) {
        let chara = gd.chara.get(CharaId::Player);
        let (weight, capacity) = chara.item_weight();

        self.info_label.set_text(&format!(
            "{:0.1}/{:0.1} kg",
            weight / 1000.0,
            capacity / 1000.0
        ));
    }

    fn do_action_for_item(&mut self, pa: &mut DoPlayerAction, il: ItemLocation) -> DialogResult {
        match self.mode {
            ItemWindowMode::List => {
                pa.request_dialog_open(DialogOpenRequest::ItemInfo { il });
                DialogResult::Continue
            }
            ItemWindowMode::PickUp => {
                pa.pick_up_item(il, 1);
                if pa.gd().is_item_on_player_tile() {
                    self.update_by_mode(pa.gd());
                    DialogResult::Continue
                } else {
                    DialogResult::Close
                }
            }
            ItemWindowMode::Drop => {
                pa.drop_item(il, 1);
                self.update_by_mode(pa.gd());
                DialogResult::Continue
            }
            ItemWindowMode::Drink => {
                pa.drink_item(il);
                DialogResult::CloseAll
            }
            ItemWindowMode::Eat => {
                pa.eat_item(il);
                DialogResult::CloseAll
            }
            ItemWindowMode::Use => {
                pa.use_item(il);
                DialogResult::CloseAll
            }
            ItemWindowMode::Release => {
                pa.release_item(il);
                DialogResult::CloseAll
            }
            ItemWindowMode::ShopBuy { .. } => {
                pa.buy_item(il);
                self.update_by_mode(pa.gd());
                DialogResult::Continue
            }
            ItemWindowMode::ShopSell => {
                pa.sell_item(il);
                self.update_by_mode(pa.gd());
                DialogResult::Continue
            }
            ItemWindowMode::Select { ref mut action, .. } => action(pa, il),
        }
    }
}

impl Window for ItemWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
        self.info_label.draw(context);
    }
}

impl DialogWindow for ItemWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        match command {
            Command::ItemInfomation => {
                let il = self.item_locations[self.list.get_current_choice() as usize];
                pa.request_dialog_open(DialogOpenRequest::ItemInfo { il })
            }
            _ => (),
        }

        let command = command.relative_to(self.rect);

        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => {
                    // Any item is selected
                    let il = self.item_locations[i as usize];
                    return self.do_action_for_item(pa, il);
                }
                ListWidgetResponse::Scrolled => {
                    self.update_by_mode(pa.gd());
                }
                _ => (),
            }
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }

    fn update(&mut self, gd: &GameData) {
        self.update_by_mode(gd);
    }
}
