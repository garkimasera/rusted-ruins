use super::commonuse::*;
use super::group_window::*;
use super::item_menu::ItemMenu;
use super::widget::*;
use crate::config::UI_CFG;
use crate::draw::border::draw_window_border;
use crate::game::extrait::*;
use crate::game::item::filter::*;
use crate::game::{DialogOpenRequest, Game, InfoGetter};
use crate::text::ToText;
use common::gamedata::*;
use common::gobj;
use sdl2::rect::Rect;

pub type ActionCallback = dyn FnMut(&mut DoPlayerAction<'_>, ItemLocation) -> DialogResult;
pub enum ItemWindowMode {
    List,
    PickUp,
    Drop,
    Throw,
    Drink,
    Eat,
    Use,
    Release,
    Read,
    Open,
    Take {
        ill: ItemListLocation,
        id: UniqueId,
    },
    Put {
        ill: ItemListLocation,
        id: UniqueId,
    },
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

impl ItemWindowMode {
    pub fn is_main_mode(&self) -> bool {
        use ItemWindowMode::*;
        matches!(
            self,
            List | Drop | Throw | Drink | Eat | Use | Release | Read | Open
        )
    }
}

pub struct ItemWindow {
    rect: Rect,
    closer: DialogCloser,
    list: ListWidget<(IconIdx, TextCache, LabelWidget)>,
    mode: ItemWindowMode,
    item_locations: Vec<ItemLocation>,
    info_label0: LabelWidget,
    info_label1: LabelWidget,
    menu: Option<super::item_menu::ItemMenu>,
    player_negotiation: u32,
}

const ITEM_WINDOW_GROUP_SIZE: u32 = 9;

pub fn create_item_window_group(game: &Game, mode: Option<ItemWindowMode>) -> GroupWindow {
    let mem_info: Vec<(MemberInfo, ChildWinCreator)> = vec![
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-list"),
                text_id: "tab_text-item_list",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::List, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-drop"),
                text_id: "tab_text-item_drop",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Drop, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-throw"),
                text_id: "tab_text-item_throw",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Throw, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-drink"),
                text_id: "tab_text-item_drink",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Drink, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-eat"),
                text_id: "tab_text-item_eat",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Eat, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-use"),
                text_id: "tab_text-item_use",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Use, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-release"),
                text_id: "tab_text-item_release",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Release, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-read"),
                text_id: "tab_text-item_read",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Read, game))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-open"),
                text_id: "tab_text-item_open",
            },
            Box::new(|game| Box::new(ItemWindow::new(ItemWindowMode::Open, game))),
        ),
    ];
    let rect: Rect = UI_CFG.item_window.rect.into();
    let i = mode.map(|mode| match mode {
        ItemWindowMode::List => 0,
        ItemWindowMode::Drop => 1,
        ItemWindowMode::Throw => 2,
        ItemWindowMode::Drink => 3,
        ItemWindowMode::Eat => 4,
        ItemWindowMode::Use => 5,
        ItemWindowMode::Release => 6,
        ItemWindowMode::Read => 7,
        ItemWindowMode::Open => 8,
        _ => unreachable!(),
    });

    GroupWindow::new(
        "item",
        ITEM_WINDOW_GROUP_SIZE,
        i,
        game,
        mem_info,
        (rect.x, rect.y),
        true,
    )
}

pub fn create_take_put_window_group(game: &Game, il: ItemLocation) -> GroupWindow {
    let id = game
        .gd
        .get_item(il)
        .0
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemAttr::Container(container) => Some(container.id()),
            _ => None,
        })
        .next()
        .expect("tried to open an item that doesn't have inner item list");

    let mem_info: Vec<(MemberInfo, ChildWinCreator)> = vec![
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-take"),
                text_id: "tab_text-item_take",
            },
            Box::new(move |game| {
                Box::new(ItemWindow::new(
                    ItemWindowMode::Take { ill: il.0, id },
                    game,
                ))
            }),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-item-put"),
                text_id: "tab_text-item_put",
            },
            Box::new(move |game| {
                Box::new(ItemWindow::new(ItemWindowMode::Put { ill: il.0, id }, game))
            }),
        ),
    ];
    let rect: Rect = UI_CFG.item_window.rect.into();

    GroupWindow::new(
        "item-put-take",
        ITEM_WINDOW_GROUP_SIZE,
        Some(0),
        game,
        mem_info,
        (rect.x, rect.y),
        true,
    )
}

impl ItemWindow {
    pub fn new(mode: ItemWindowMode, game: &Game) -> ItemWindow {
        let player_negotiation = game
            .gd
            .chara
            .get(CharaId::Player)
            .skill_level(SkillKind::Negotiation);
        let rect = UI_CFG.item_window.rect.into();
        let n_row = UI_CFG.item_window.n_row;
        let list_h = UI_CFG.list_widget.h_row_default;

        let mut item_window = ItemWindow {
            rect,
            closer: DialogCloser::new(rect),
            list: ListWidget::with_scroll_bar(
                (0i32, 0i32, rect.w as u32, n_row * list_h),
                UI_CFG.item_window.column_pos.clone(),
                n_row,
                true,
            ),
            mode,
            item_locations: Vec::new(),
            info_label0: LabelWidget::new(UI_CFG.item_window.info_label_rect0, "", FontKind::M),
            info_label1: LabelWidget::new(UI_CFG.item_window.info_label_rect1, "", FontKind::M)
                .right(),
            menu: None,
            player_negotiation,
        };
        item_window.update_by_mode(&game.gd);
        item_window
    }

    pub fn new_select(
        ill: ItemListLocation,
        filter: ItemFilter,
        action: Box<ActionCallback>,
        pa: &mut DoPlayerAction<'_>,
    ) -> ItemWindow {
        let mode = ItemWindowMode::Select {
            ill,
            filter,
            action,
        };
        ItemWindow::new(mode, pa.game())
    }

    pub fn new_select_and_equip(
        cid: CharaId,
        slot: (EquipSlotKind, u8),
        pa: &mut DoPlayerAction<'_>,
    ) -> ItemWindow {
        let equip_selected_item = move |pa: &mut DoPlayerAction<'_>, il: ItemLocation| {
            pa.change_equipment(cid, slot, il);
            DialogResult::Close
        };

        ItemWindow::new_select(
            ItemListLocation::Chara { cid },
            ItemFilter::new().equip_slot_kind(slot.0),
            Box::new(equip_selected_item),
            pa,
        )
    }

    fn update_by_mode(&mut self, gd: &GameData) {
        let ill_player = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
        let ill_ground = ItemListLocation::OnMap {
            mid: gd.get_current_mapid(),
            pos: gd.player_pos(),
        };

        match &self.mode {
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
            ItemWindowMode::Throw => {
                let player_str = gd.chara.get(CharaId::Player).attr.str;
                let filter = ItemFilter::new().throwable(Some(player_str));
                let filtered_list = gd.get_filtered_item_list(ill_player, filter);
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
            ItemWindowMode::Read => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().readable(true),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::Open => {
                let filtered_list = gd.get_merged_filtered_item_list(
                    ill_ground,
                    ill_player,
                    ItemFilter::new().container(true),
                );
                self.update_list(filtered_list);
            }
            ItemWindowMode::Put { ill, id } => {
                let il = gd.find_container_item(*ill, *id).unwrap();
                let item_obj = gd.get_item(il).0.obj();
                let (selector, function) =
                    if let Some(ItemObjAttr::Container {
                        selector, function, ..
                    }) = find_attr!(item_obj, ItemObjAttr::Container)
                    {
                        (selector, function)
                    } else {
                        panic!();
                    };

                let mut item_filter = ItemFilter::new()
                    .deny_container()
                    .selector(selector.clone());

                if let ContainerFunction::Converter { kind } = function {
                    item_filter = item_filter.convertable_by_container(kind);
                }

                let filtered_list =
                    gd.get_merged_filtered_item_list(ill_ground, ill_player, item_filter);
                self.update_list(filtered_list);
            }
            ItemWindowMode::Take { ill, id } => {
                let il = gd.find_container_item(*ill, *id).unwrap();
                let ill_in_container = ItemListLocation::in_container(il);
                let filtered_list = gd.get_filtered_item_list(ill_in_container, ItemFilter::new());
                self.update_list(filtered_list);
            }
            ItemWindowMode::ShopBuy { cid } => {
                let ill = ItemListLocation::Shop { cid: *cid };
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
                let filtered_list = gd.get_filtered_item_list(*ill, filter.clone());
                self.update_list(filtered_list);
            }
        }
        self.update_label(gd);
    }

    fn update_list(&mut self, list: FilteredItemList<'_>) {
        self.list.set_n_item(list.clone().count() as u32);

        let player_negotiation = self.player_negotiation;
        let mode = &self.mode;

        self.item_locations.clear();
        for (il, _, _) in list.clone() {
            self.item_locations.push(il);
        }

        let window_width = self.rect.width();

        self.list.update_rows_by_func(move |i| {
            let (_, item, n_item) = list.clone().nth(i as usize).unwrap();

            let item_text = format!("{} x {}", item.to_text(), n_item);

            let w = item.w() as f32 * n_item as f32;

            // Information displayed in the right column
            let additional_info = match mode {
                ItemWindowMode::ShopBuy { .. } => {
                    format!(
                        "{}{}",
                        crate::text::img_inline::SILVER,
                        item.buying_price(player_negotiation)
                    )
                }
                ItemWindowMode::ShopSell => {
                    format!(
                        "{}{}",
                        crate::text::img_inline::SILVER,
                        item.selling_price(player_negotiation)
                    )
                }
                _ => format!("{:.1}kg", w / 1000.0),
            };

            let t1 = TextCache::new(item_text, FontKind::M, UI_CFG.color.normal_font);
            let w = window_width
                - UI_CFG.item_window.column_pos[2] as u32
                - UI_CFG.vscroll_widget.width;
            let t2 = LabelWidget::new(
                Rect::new(0, 0, w, UI_CFG.list_widget.h_row_default),
                &additional_info,
                FontKind::M,
            )
            .right();

            (item.icon(), t1, t2)
        });
    }

    fn update_label(&mut self, gd: &GameData) {
        let chara = gd.chara.get(CharaId::Player);
        let (weight, capacity) = chara.item_weight();

        self.info_label0.set_text(&format!(
            "{:0.1}/{:0.1} kg",
            weight / 1000.0,
            capacity / 1000.0
        ));

        match self.mode {
            ItemWindowMode::ShopBuy { .. } | ItemWindowMode::ShopSell { .. } => {
                self.info_label1.set_text(&format!(
                    "{}{}",
                    crate::text::img_inline::SILVER,
                    gd.player.money()
                ));
            }
            _ => (),
        }
    }

    fn do_action_for_item(
        &mut self,
        pa: &mut DoPlayerAction<'_>,
        il: ItemLocation,
    ) -> DialogResult {
        match self.mode {
            ItemWindowMode::List => {
                pa.request_dialog_open(DialogOpenRequest::ItemInfo { il });
                DialogResult::Continue
            }
            ItemWindowMode::PickUp => {
                pa.pick_up_item(il, ItemMoveNum::All);
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
            ItemWindowMode::Throw => {
                pa.throw_item(il);
                DialogResult::CloseAll
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
            ItemWindowMode::Read => {
                if pa.read_item(il) {
                    DialogResult::Continue
                } else {
                    DialogResult::CloseAll
                }
            }
            ItemWindowMode::Open => {
                return DialogResult::OpenChildDialog(Box::new(create_take_put_window_group(
                    pa.game(),
                    il,
                )));
            }
            ItemWindowMode::Take { .. } => {
                pa.move_item(il, ItemListLocation::PLAYER, 1);
                self.update_by_mode(pa.gd());
                DialogResult::Continue
            }
            ItemWindowMode::Put { ill, id } => {
                let container_il = pa.gd().find_container_item(ill, id).unwrap();
                let ill_in_container = ItemListLocation::in_container(container_il);
                pa.move_item(il, ill_in_container, 1);
                self.update_by_mode(pa.gd());
                DialogResult::Continue
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
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.list.draw(context);
        self.info_label0.draw(context);
        self.info_label1.draw(context);
        if let Some(menu) = self.menu.as_mut() {
            menu.draw(context, game, anim);
        }
    }
}

impl DialogWindow for ItemWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if let Some(menu) = self.menu.as_mut() {
            match menu.process_command(command, pa) {
                DialogResult::Special(SpecialDialogResult::ItemListUpdate) => {
                    self.menu = None;
                    self.update_by_mode(pa.gd());
                    return DialogResult::Continue;
                }
                DialogResult::Close => {
                    self.menu = None;
                    return DialogResult::Continue;
                }
                DialogResult::CloseAll => {
                    self.menu = None;
                    return DialogResult::CloseAll;
                }
                _ => {
                    return DialogResult::Continue;
                }
            }
        }

        let cursor_pos = if let Command::MouseButtonUp { x, y, .. } = command {
            Some((*x, *y))
        } else {
            None
        };

        closer!(self, command, self.mode.is_main_mode());

        if command == &Command::ItemInformation {
            let il = self.item_locations[self.list.get_current_choice() as usize];
            pa.request_dialog_open(DialogOpenRequest::ItemInfo { il })
        }

        let command = command.relative_to(self.rect);

        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => {
                    // Any item is selected
                    let il = self.item_locations[i as usize];
                    return self.do_action_for_item(pa, il);
                }
                ListWidgetResponse::SelectForMenu(i) => {
                    // Item selected to open menu
                    let il = self.item_locations[i as usize];
                    self.menu = Some(ItemMenu::new(pa.gd(), &self.mode, il, cursor_pos));
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

    fn callback_child_closed(
        &mut self,
        _result: Option<DialogCloseValue>,
        pa: &mut DoPlayerAction<'_>,
    ) -> DialogResult {
        self.update_by_mode(pa.gd());
        DialogResult::Continue
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }

    fn update(&mut self, gd: &GameData) {
        self.update_by_mode(gd);
    }

    fn tab_switched(&mut self) {
        self.menu = None;
    }
}
