use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::draw::border::draw_rect_border;
use common::objholder::ItemIdx;
use common::gamedata::CreationKind;
use common::gobj;
use rules::RULES;

pub struct CreationWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, TextCache)>,
    kind: CreationKind,
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
                26,
                true,
                false,
            ),
            kind,
        };

        w.update(kind);
        w
    }

    pub fn update(&mut self, kind: CreationKind) {
        self.kind = kind;

        let recipes = RULES.creation.recipes(kind);

        let items: Vec<(IconIdx, TextCache)> = recipes.iter().map(|r| {
            let idx: ItemIdx = gobj::id_to_idx(&r.product);
            let t = TextCache::one(crate::text::obj_txt(&r.product), FontKind::M, UI_CFG.color.normal_font.into());
            (IconIdx::Item(idx), t)
        }).collect();

        self.list.set_items(items);
    }
}

impl Window for CreationWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for CreationWindow {
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
