
use sdl2::rect::Rect;
use config::UI_CFG;
use super::commonuse::*;
use super::widget::*;
use text;

pub struct ExitWindow {
    rect: Rect,
    label: LabelWidget,
    answer_list: ListWidget,
}

impl ExitWindow {
    pub fn new() -> ExitWindow {
        let choices = vec!["Yes".to_owned(), "No".to_owned()];
        let rect = UI_CFG.exit_window.rect.into();
        ExitWindow {
            rect: rect,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), text::ui_txt("dialog.exit"), rect.w as u32),
            answer_list: ListWidget::single((0, UI_CFG.exit_window.list_y, rect.w as u32, 0), choices),
        }
    }
}

impl Window for ExitWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);

        self.label.draw(canvas, sv);
        self.answer_list.draw(canvas, sv);
    }
}

impl DialogWindow for ExitWindow {
    fn process_command(&mut self, command: Command, _pa: DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(0) => { // Yes for quit game
                    return DialogResult::Quit;
                },
                ListWidgetResponse::Select(1) => { // No for quit game
                    return DialogResult::Close;
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


