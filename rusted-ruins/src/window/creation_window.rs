use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::draw::border::draw_window_border;
use crate::text::{obj_txt, ToText};
use common::gamedata::{CreationKind, GameData, ItemLocation, Recipe};
use common::gobj;
use common::objholder::*;
use rules::RULES;

pub struct CreationWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, TextCache)>,
    recipes: Vec<&'static Recipe>,
    kind: CreationKind,
    detail_dialog: Option<CreationDetailDialog>,
}

impl CreationWindow {
    pub fn new(kind: CreationKind) -> CreationWindow {
        let c = &UI_CFG.creation_window;
        let rect: Rect = c.rect.into();

        let mut w = CreationWindow {
            rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                c.column_pos.clone(),
                c.n_row,
                true,
            ),
            recipes: Vec::new(),
            kind,
            detail_dialog: None,
        };

        w.update(kind);
        w
    }

    pub fn update(&mut self, kind: CreationKind) {
        self.kind = kind;

        let recipes = RULES.creation.recipes(kind);
        self.recipes = recipes.iter().map(|r| r).collect(); // TODO: Recipe filtering

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
            self.detail_dialog = Some(CreationDetailDialog::new(pa.gd(), self.recipes[i as usize]));
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
    recipe: &'static Recipe,
    product_name: LabelWidget,
    list: ListWidget<(IconIdx, TextCache, IconIdx)>,
    il: Vec<Option<ItemLocation>>,
    possible: bool,
}

impl CreationDetailDialog {
    fn new(gd: &GameData, recipe: &'static Recipe) -> CreationDetailDialog {
        let c = &UI_CFG.creation_detail_dialog;
        let rect: Rect = c.rect.into();

        let mut list = ListWidget::new(
            (0i32, c.list_margin, rect.w as u32, rect.h as u32),
            c.column_pos.clone(),
            c.n_row,
            false,
        );

        let icon_idx_ok: UIImgIdx = gobj::id_to_idx("!icon-ok");
        let icon_idx_ng: UIImgIdx = gobj::id_to_idx("!icon-ng");

        let autopicked_items = crate::game::creation::item_auto_pick(gd, recipe);
        let mut list_items: Vec<(IconIdx, TextCache, IconIdx)> = autopicked_items
            .iter()
            .enumerate()
            .map(|(i, il)| {
                if let Some(il) = il {
                    let item = gd.get_item(*il);
                    let text = TextCache::one(
                        format!("{} x {}", item.0.to_text(), item.1),
                        FontKind::M,
                        UI_CFG.color.normal_font.into(),
                    );
                    (IconIdx::Item(item.0.idx), text, IconIdx::UIImg(icon_idx_ok))
                } else {
                    let ingredient_idx: ItemIdx = gobj::id_to_idx(&recipe.ingredients[i]);
                    let text = TextCache::one(
                        format!("{} x {}", obj_txt(&recipe.ingredients[i]), 0),
                        FontKind::M,
                        UI_CFG.color.normal_font.into(),
                    );
                    (
                        IconIdx::Item(ingredient_idx),
                        text,
                        IconIdx::UIImg(icon_idx_ng),
                    )
                }
            })
            .collect();
        let possible = autopicked_items.iter().all(|a| a.is_some());
        list_items.push((
            IconIdx::UIImg(if possible { icon_idx_ok } else { icon_idx_ng }),
            TextCache::one(
                crate::text::ui_txt("creation.start"),
                FontKind::M,
                UI_CFG.color.normal_font.into(),
            ),
            IconIdx::UIImg(gobj::id_to_idx("!")),
        ));
        list.set_items(list_items);

        CreationDetailDialog {
            rect,
            recipe,
            product_name: LabelWidget::new(c.product_name, &obj_txt(&recipe.product), FontKind::M),
            list,
            il: autopicked_items,
            possible,
        }
    }
}

impl Window for CreationDetailDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.product_name.draw(context);
        self.list.draw(context);
    }
}

impl DialogWindow for CreationDetailDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        let command = command.relative_to(self.rect);
        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            // Any item is selected
            if self.possible && self.recipe.ingredients.len() as u32 == i {
                let il: Vec<ItemLocation> = self.il.iter().map(|a| a.unwrap()).collect();
                pa.start_creation(self.recipe, il);
                return DialogResult::CloseAll;
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
}
