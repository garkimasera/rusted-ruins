
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
use common::gamedata::chara::CharaId;
use text;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget,
    n_row: u32,
    current_page: u32,
    cid: CharaId,
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
        };
        equip_window.update_list(pa);
        equip_window
    }

    fn update_list(&mut self, pa: DoPlayerAction) {
        
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

