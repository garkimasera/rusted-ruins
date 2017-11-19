
use sdl2::rect::Rect;
use config::UI_CFG;
use game::DoPlayerAction;
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;

pub struct YesNoDialog {
    rect: Rect,
    label: LabelWidget,
    answer_list: ListWidget,
    action_on_yes: Box<FnMut(&mut DoPlayerAction) -> DialogResult + 'static>
}

impl YesNoDialog {
    pub fn new<F>(msg: &str, f: F) -> YesNoDialog
        where F: FnMut(&mut DoPlayerAction) -> DialogResult + 'static {
        
        let choices = vec!["Yes".to_owned(), "No".to_owned()];
        let rect = UI_CFG.exit_window.rect.into();
        YesNoDialog {
            rect: rect,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), &msg, FontKind::M, rect.w as u32),
            answer_list: ListWidget::single((0, UI_CFG.exit_window.list_y, rect.w as u32, 0), choices),
            action_on_yes: Box::new(f),
        }
    }
}

impl Window for YesNoDialog {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);

        self.label.draw(canvas, sv);
        self.answer_list.draw(canvas, sv);
    }
}

impl DialogWindow for YesNoDialog {
    fn process_command(&mut self, command: Command, mut pa: DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(0) => { // Yes
                    return (self.action_on_yes)(&mut pa);
                }
                ListWidgetResponse::Select(1) => { // No
                    return DialogResult::Close;
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

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

