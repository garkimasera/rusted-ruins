
use window::{Window, DialogWindow, DialogResult};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdlvalues::*;
use game::{Game, Animation, Command, DoPlayerAction, InfoGetter};
use config::UI_CFG;
use draw::border::draw_rect_border;
use eventhandler::InputMode;
use super::widget::*;
use common::gobj;
use common::gamedata::item::ItemKind;
use common::gamedata::chara::CharaId;
use text;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget,
    n_row: u32,
    current_page: u32,
    cid: CharaId,
    slots: Vec<(ItemKind, u8)>
}

impl EquipWindow {
    pub fn new(pa: DoPlayerAction, cid: CharaId) -> EquipWindow {
        let rect = UI_CFG.equip_window.rect.into();
        
        let mut equip_window = EquipWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRow::IconStr(vec![]), vec![0, 26]),
            n_row: UI_CFG.equip_window.n_row,
            current_page: 0,
            cid: cid,
            slots: Vec::new(),
        };
        equip_window.update_list(pa);
        equip_window
    }

    fn update_list(&mut self, pa: DoPlayerAction) {
        let mut rows: Vec<(IconIdx, String)> = Vec::new();
        let equips = pa.gd().get_equip_list(self.cid);
        self.slots.clear();

        for (ik, ik_i, item) in equips.slot_iter() {
            if let Some(item) = item {
                let item_text = format!(
                    "{} ({:?})",
                    text::obj_txt(&gobj::get_obj(item.idx).id).to_owned(),
                    ik);
                rows.push((IconIdx::Item(item.idx), item_text));
            } else {
                rows.push((IconIdx::Item(::common::objholder::ItemIdx(0)), "Empty".to_owned()));
            }
            self.slots.push((ik, ik_i));
        }
        self.list.set_rows(ListRow::IconStr(rows));
    }
}

impl Window for EquipWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.list.draw(canvas, sv);
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                    
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        self.list.process_command(&command);
        
        match command {
            Command::Cancel => {
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

