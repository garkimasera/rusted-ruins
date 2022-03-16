use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::text::obj_txt;
use common::gamedata::BuildObj;
use common::gamedata::ItemLocation;
use sdl2::rect::Rect;

pub struct BuildObjDialog {
    rect: Rect,
    closer: DialogCloser,
    list: ListWidget<TextCache>,
    build_objs: Vec<(BuildObj, u32)>,
    il: ItemLocation,
}

impl BuildObjDialog {
    pub fn new(il: ItemLocation) -> BuildObjDialog {
        let cfg = &UI_CFG.build_obj_dialog;
        let rect: Rect = cfg.rect.into();
        let mut list = ListWidget::with_scroll_bar(
            Rect::new(0, 0, rect.width(), rect.height()),
            cfg.column_pos.clone(),
            cfg.n_row,
            false,
        );
        let build_objs = crate::game::building::build_obj_list();
        let items: Vec<TextCache> = build_objs
            .iter()
            .map(|(build_obj, _)| {
                let item_text = match build_obj {
                    BuildObj::Tile(id) => obj_txt(id),
                    BuildObj::Wall(id) => obj_txt(id),
                };
                TextCache::new(item_text, FontKind::M, UI_CFG.color.normal_font)
            })
            .collect();
        list.set_items(items);

        BuildObjDialog {
            rect,
            closer: DialogCloser::new(rect),
            list,
            build_objs,
            il,
        }
    }
}

impl Window for BuildObjDialog {
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

impl DialogWindow for BuildObjDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if *command == Command::Cancel {
            return DialogResult::Close;
        }

        closer!(self, command);
        let command = command.relative_to(self.rect);

        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            let build_obj = self.build_objs[i as usize].0.clone();
            pa.select_build_obj(self.il, build_obj);
            return DialogResult::Close;
        }

        DialogResult::Continue
    }
}
