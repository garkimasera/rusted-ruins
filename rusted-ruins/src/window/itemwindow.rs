
use window::{Window, DialogWindow, DialogResult};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdlvalues::*;
use game::{Game, Animation, Command, DoPlayerAction};
use config::UI_CFG;
use draw::border::draw_rect_border;
use eventhandler::InputMode;
use super::widget::*;
use common::gobj;

pub struct ItemWindow {
    rect: Rect,
    list: ListWidget,
}

impl ItemWindow {
    pub fn new() -> ItemWindow {
        let rect = UI_CFG.item_window.rect.into();
        let idx: ::common::objholder::ItemIdx = gobj::id_to_idx("!plank");
        let v = vec![(IconIdx::Item(idx), "木材".to_owned()), (IconIdx::Item(idx), "良い木材".to_owned())];
        ItemWindow {
            rect: rect,
            list: ListWidget::new((0i32, 0i32, rect.w as u32, rect.h as u32), ListRow::IconStr(v), vec![0, 26]),
        }
    }
}

impl Window for ItemWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.list.draw(canvas, sv);
    }
}

impl DialogWindow for ItemWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
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
