use super::commonuse::*;
use super::group_window::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::draw::border::draw_window_border;
use crate::game::creation;
use crate::text::{misc_txt, obj_txt, ui_txt, ToText};
use common::gamedata::*;
use common::gobj;
use common::objholder::*;

pub fn create_creation_window_group(
    game: &Game,
    creation_kind: Option<CreationKind>,
) -> GroupWindow {
    let mem_info = vec![
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-art"),
            text_id: "tab_text-creation_art",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Art)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-construction"),
            text_id: "tab_text-creation_construction",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Construction)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-cooking"),
            text_id: "tab_text-creation_cooking",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Cooking)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-craft"),
            text_id: "tab_text-creation_craft",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Craft)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-pharmacy"),
            text_id: "tab_text-creation_pharmacy",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Pharmacy)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-creation-smith"),
            text_id: "tab_text-creation_smith",
            creator: |game| Box::new(CreationWindow::new(&game.gd, CreationKind::Smith)),
        },
    ];
    let rect: Rect = UI_CFG.creation_window.rect.into();
    let i = if let Some(creation_kind) = creation_kind {
        match creation_kind {
            CreationKind::Art => 0,
            CreationKind::Construction => 1,
            CreationKind::Cooking => 2,
            CreationKind::Craft => 3,
            CreationKind::Pharmacy => 4,
            CreationKind::Smith => 5,
        }
    } else {
        0
    };
    GroupWindow::new(mem_info.len() as u32, i, game, mem_info, (rect.x, rect.y))
}

pub struct CreationWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, TextCache)>,
    recipes: Vec<&'static Recipe>,
    kind: CreationKind,
    detail_dialog: Option<CreationDetailDialog>,
    escape_click: bool,
}

impl CreationWindow {
    pub fn new(gd: &GameData, kind: CreationKind) -> CreationWindow {
        let c = &UI_CFG.creation_window;
        let rect: Rect = c.rect.into();

        let mut w = CreationWindow {
            rect,
            list: ListWidget::with_scroll_bar(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                c.column_pos.clone(),
                c.n_row,
                true,
            ),
            recipes: Vec::new(),
            kind,
            detail_dialog: None,
            escape_click: false,
        };

        w.update(gd, kind);
        w
    }

    pub fn update(&mut self, gd: &GameData, kind: CreationKind) {
        self.kind = kind;
        self.recipes = creation::available_recipes(gd, kind);

        let items: Vec<(IconIdx, TextCache)> = self
            .recipes
            .iter()
            .map(|r| {
                let idx: ItemIdx = gobj::id_to_idx(&r.product);
                let t = TextCache::one(
                    obj_txt(&r.product),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                (IconIdx::Item(idx), t)
            })
            .collect();

        self.list.set_items(items);
    }
}

impl Window for CreationWindow {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        if let Some(detail_dialog) = self.detail_dialog.as_mut() {
            detail_dialog.draw(context, game, anim);
        } else {
            draw_window_border(context, self.rect);
            self.list.draw(context);
        }
    }
}

impl DialogWindow for CreationWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        if let Some(detail_dialog) = self.detail_dialog.as_mut() {
            let result = detail_dialog.process_command(command, pa);
            match result {
                DialogResult::Close => {
                    self.detail_dialog = None;
                    return DialogResult::Continue;
                }
                _ => (),
            }
            return result;
        }

        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            // Any item is selected
            self.detail_dialog = Some(CreationDetailDialog::new(
                pa.gd(),
                self.recipes[i as usize],
                self.kind,
            ));
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
}

pub struct CreationDetailDialog {
    rect: Rect,
    kind: CreationKind,
    recipe: &'static Recipe,
    available_material: Vec<(ItemIdx, u32)>,
    selected_material: usize,
    available_skill_facility: bool,
    product_name: LabelWidget,
    start_button: Option<ButtonWidget>,
    cancel_button: ButtonWidget,
    list: ListWidget<(IconIdx, TextCache, TextCache)>,
    escape_click: bool,
    facility_ok_icon: ImageWidget,
    facility_label: LabelWidget,
    enough_ingredients_icon: ImageWidget,
    enough_ingredients_label: LabelWidget,
    required_skill_icon: ImageWidget,
    required_skill_label: LabelWidget,
}

impl CreationDetailDialog {
    fn new(gd: &GameData, recipe: &'static Recipe, kind: CreationKind) -> CreationDetailDialog {
        let available_material = creation::available_material(gd, recipe, ItemListLocation::PLAYER);
        let c = &UI_CFG.creation_detail_dialog;
        let rect: Rect = c.rect.into();

        let list = ListWidget::new(
            (0i32, c.list_margin, rect.w as u32, rect.h as u32),
            c.column_pos.clone(),
            c.n_row,
            false,
        );

        let mut possible = true;
        let facility_item = if let Some(facility_type) = recipe.facility.as_ref() {
            let facility_item = crate::game::map::search::search_facility(gd, &facility_type);
            if facility_item.is_none() {
                possible = false;
            }
            facility_item
        } else {
            None
        };

        let required_skill =
            crate::game::creation::enough_skill(gd.chara.get(CharaId::Player), recipe, kind);
        if !required_skill {
            possible = false;
        }
        let cancel_button = ButtonWidget::new(
            c.cancel_button_rect,
            &ui_txt("button_text-creation-cancel"),
            FontKind::M,
        );

        let icon_id = if facility_item.is_some() || recipe.facility.is_none() {
            "!icon-ok"
        } else {
            "!icon-ng"
        };
        let facility_ok_icon = ImageWidget::ui_img(c.facility_ok_icon_rect, icon_id);
        let label = if let Some(facility_item) = facility_item {
            format!(
                "{}: {}",
                ui_txt("label_text-creation-use-facility"),
                facility_item.to_text()
            )
        } else if let Some(facility_name) = recipe.facility.as_ref() {
            let text_id = format!("facility-{}", facility_name);
            format!(
                "{}: {}",
                ui_txt("label_text-creation-required-facility"),
                misc_txt(&text_id),
            )
        } else {
            ui_txt("label_text-creation-no-required-facility")
        };
        let facility_label = LabelWidget::new(c.facility_label_rect, &label, FontKind::M);

        let (enough_ingredients_icon, enough_ingredients_label) =
            ("!icon-ng", "label_text-creation-not-enough-ingredients");
        let enough_ingredients_icon =
            ImageWidget::ui_img(c.enough_ingredients_icon_rect, enough_ingredients_icon);
        let enough_ingredients_label = LabelWidget::new(
            c.enough_ingredients_label_rect,
            &ui_txt(enough_ingredients_label),
            FontKind::M,
        );

        let icon_id = if required_skill {
            "!icon-ok"
        } else {
            "!icon-ng"
        };
        let required_skill_icon = ImageWidget::ui_img(c.required_skill_icon_rect, icon_id);
        let required_skill_label = format!(
            "{} ({}: {})",
            ui_txt("label_text-creation-required_skill"),
            kind.to_text(),
            recipe.difficulty
        );
        let required_skill_label = LabelWidget::new(
            c.required_skill_label_rect,
            &required_skill_label,
            FontKind::M,
        );

        let mut dialog = CreationDetailDialog {
            rect,
            kind,
            recipe,
            available_material,
            selected_material: 0,
            available_skill_facility: possible,
            product_name: LabelWidget::new(c.product_name, &obj_txt(&recipe.product), FontKind::M),
            list,
            start_button: None,
            cancel_button,
            escape_click: false,
            facility_ok_icon,
            facility_label,
            enough_ingredients_icon,
            enough_ingredients_label,
            required_skill_icon,
            required_skill_label,
        };
        dialog.update(gd);
        dialog
    }

    fn update(&mut self, gd: &GameData) {
        let c = &UI_CFG.creation_detail_dialog;
        let mut enough_ingredients = true;
        let item_list = gd.get_item_list(ItemListLocation::PLAYER);

        let list_items: Vec<(IconIdx, TextCache, TextCache)> = self
            .recipe
            .ingredients
            .iter()
            .map(|(ingredient, n)| {
                let item_id = if let Some(group) = creation::material_group(ingredient) {
                    if let Some((idx, _)) = self.available_material.get(self.selected_material) {
                        gobj::idx_to_id(*idx)
                    } else {
                        // No available item for this ingredient material group
                        enough_ingredients = false;
                        let icon_idx = gobj::id_to_idx("!icon-question");
                        let material_group_name = crate::text::prefix::material_group(group);
                        let msg = ui_txt_format!(
                            "list_item_text-creation-no_ingredient"; group=material_group_name);
                        let item_name =
                            TextCache::one(msg, FontKind::M, UI_CFG.color.normal_font.into());
                        let item_n = TextCache::one(
                            format!("0/{}", n),
                            FontKind::M,
                            UI_CFG.color.normal_font.into(),
                        );
                        return (IconIdx::UIImg(icon_idx), item_name, item_n);
                    }
                } else {
                    ingredient
                };
                let idx: ItemIdx = gobj::id_to_idx(item_id);
                let total = item_list.count(idx);
                if total < *n {
                    enough_ingredients = false;
                }
                let group = if let Some(group) = creation::material_group(ingredient) {
                    format!("({}) ", crate::text::prefix::material_group(group))
                } else {
                    "".into()
                };
                let item_name = TextCache::one(
                    format!("{}{}", group, obj_txt(item_id)),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                let item_n = TextCache::one(
                    format!("{}/{}", total, n),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                (IconIdx::Item(idx), item_name, item_n)
            })
            .collect();
        self.list.set_items(list_items);

        let (enough_ingredients_icon, enough_ingredients_label) = if enough_ingredients {
            ("!icon-ok", "label_text-creation-enough-ingredients")
        } else {
            ("!icon-ng", "label_text-creation-not-enough-ingredients")
        };
        self.enough_ingredients_icon =
            ImageWidget::ui_img(c.enough_ingredients_icon_rect, enough_ingredients_icon);
        self.enough_ingredients_label = LabelWidget::new(
            c.enough_ingredients_label_rect,
            &ui_txt(enough_ingredients_label),
            FontKind::M,
        );
        self.start_button = if enough_ingredients && self.available_skill_facility {
            Some(ButtonWidget::new(
                c.start_button_rect,
                &ui_txt("button_text-creation-start"),
                FontKind::M,
            ))
        } else {
            None
        };
    }
}

impl Window for CreationDetailDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.product_name.draw(context);
        self.list.draw(context);
        self.facility_ok_icon.draw(context);
        self.facility_label.draw(context);
        self.enough_ingredients_icon.draw(context);
        self.enough_ingredients_label.draw(context);
        self.required_skill_icon.draw(context);
        self.required_skill_label.draw(context);
        if let Some(start_button) = self.start_button.as_mut() {
            start_button.draw(context);
        }
        self.cancel_button.draw(context);
    }
}

impl DialogWindow for CreationDetailDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            // Any item is selected
            if creation::material_group(&self.recipe.ingredients[i as usize].0).is_none() {
                return DialogResult::Continue;
            }
            // Update by selected material
            self.selected_material += 1;
            if self.selected_material >= self.available_material.len() {
                self.selected_material = 0;
            }
            self.update(pa.gd());
            return DialogResult::Continue;
        }

        if let Some(start_button) = self.start_button.as_mut() {
            if let Some(_) = start_button.process_command(&command) {
                let material_to_use = self
                    .available_material
                    .get(self.selected_material)
                    .map(|(idx, _)| *idx);
                // If start button is pressed, start creation.
                pa.start_creation(
                    self.kind,
                    self.recipe,
                    ItemListLocation::PLAYER,
                    false,
                    material_to_use,
                );
                return DialogResult::CloseAll;
            }
        }

        if let Some(_) = self.cancel_button.process_command(&command) {
            return DialogResult::Close;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
