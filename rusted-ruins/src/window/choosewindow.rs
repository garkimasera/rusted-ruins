
use sdl2::rect::Rect;
use config::UI_CFG;
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use text;

pub struct ChooseWindow {
    rect: Rect,
    answer_list: ListWidget,
    default_choose: Option<u32>,
}

impl ChooseWindow {
    pub fn new(y: i32, choices: Vec<String>, default_choose: Option<u32>) -> ChooseWindow {
        let rect = Rect::new(10, y, 100, 100);
        ChooseWindow {
            rect: rect,
            answer_list: ListWidget::single((0, UI_CFG.exit_window.list_y, rect.w as u32, 0), choices),
            default_choose: default_choose,
        }
    }
}

impl Window for ChooseWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        
        self.answer_list.draw(canvas, sv);
    }
}

impl DialogWindow for ChooseWindow {
    fn process_command(&mut self, command: Command, _pa: DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(n) => {
                    return DialogResult::CloseWithValue(Box::new(n));
                },
                _ => (),
            }
            return DialogResult::Continue;
        }
        
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

