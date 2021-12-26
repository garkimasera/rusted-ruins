use super::commonuse::*;
use super::widget::*;
use super::winpos::WindowPos;
use crate::config::SCREEN_CFG;
use crate::text::ui_txt;

/// Player chooses one item from list.
/// The choices cannot be changed.
/// This handles text list only.
pub struct ChooseWindow {
    winpos: WindowPos,
    rect: Option<Rect>,
    answer_list: TextListWidget,
    default_behavior: DefaultBehavior,
    callbacks: Vec<Box<dyn FnMut(&mut DoPlayerAction<'_>) + 'static>>,
    mainwin_cursor: bool,
    escape_click: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DefaultBehavior {
    Close,
    Ignore,
}

impl ChooseWindow {
    pub fn new<S: AsRef<str>>(
        winpos: WindowPos,
        choices: Vec<S>,
        default_behavior: DefaultBehavior,
    ) -> ChooseWindow {
        ChooseWindow {
            winpos,
            rect: None,
            answer_list: TextListWidget::text_choices((0, 0, 0, 0), choices),
            default_behavior,
            callbacks: Vec::new(),
            mainwin_cursor: false,
            escape_click: false,
        }
    }

    /// Create ChooseWindow with two choices, yes and no
    /// default_choose: When Esc is inputed, which choice will be returned
    pub fn with_yesno(winpos: WindowPos, default_behavior: DefaultBehavior) -> ChooseWindow {
        let choices = vec!["Yes".to_owned(), "No".to_owned()];
        ChooseWindow::new(winpos, choices, default_behavior)
    }

    /// Create menu with callbacks
    pub fn menu(
        winpos: WindowPos,
        text_ids: Vec<&str>,
        callbacks: Vec<Box<dyn FnMut(&mut DoPlayerAction<'_>) + 'static>>,
    ) -> ChooseWindow {
        let choices: Vec<String> = text_ids.iter().map(|tid| ui_txt(tid)).collect();
        ChooseWindow {
            winpos,
            rect: None,
            answer_list: TextListWidget::text_choices((0, 0, 0, 0), choices),
            default_behavior: DefaultBehavior::Close,
            callbacks,
            mainwin_cursor: true,
            escape_click: false,
        }
    }

    pub fn set_winpos(&mut self, winpos: WindowPos) {
        self.winpos = winpos;
    }
}

impl Window for ChooseWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        // Update window size
        let list_widget_size = self.answer_list.adjust_widget_size(context.sv);
        let left_top_point = self
            .winpos
            .calc_left_top(list_widget_size.0, list_widget_size.1);
        let mut rect = Rect::new(
            left_top_point.0,
            left_top_point.1,
            list_widget_size.0,
            list_widget_size.1,
        );

        if rect.right() > SCREEN_CFG.screen_w as i32 {
            rect.offset(-(rect.right() - SCREEN_CFG.screen_w as i32), 0)
        }

        // Drawing
        draw_window_border(context, rect);

        self.answer_list.draw(context);
        self.rect = Some(rect);
    }
}

impl DialogWindow for ChooseWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        let rect = if let Some(rect) = self.rect {
            rect
        } else {
            return DialogResult::Continue;
        };
        let command = command.relative_to(rect);
        if let Some(response) = self.answer_list.process_command(&command) {
            if let ListWidgetResponse::Select(n) = response {
                if self.callbacks.is_empty() {
                    return DialogResult::CloseWithValue(DialogCloseValue::Index(n));
                } else {
                    self.callbacks.get_mut(n as usize).unwrap()(pa);
                    return DialogResult::Close;
                }
            }
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => match self.default_behavior {
                DefaultBehavior::Close => DialogResult::Close,
                DefaultBehavior::Ignore => DialogResult::Continue,
            },
            Command::MouseButtonUp { x, y, .. } if x < 0 || x >= rect.w || y < 0 || y >= rect.h => {
                match self.default_behavior {
                    DefaultBehavior::Close => {
                        if self.escape_click {
                            DialogResult::Close
                        } else {
                            DialogResult::Continue
                        }
                    }
                    DefaultBehavior::Ignore => DialogResult::Continue,
                }
            }
            Command::MouseButtonDown { x, y, .. }
                if x < 0 || x >= rect.w || y < 0 || y >= rect.h =>
            {
                self.escape_click = true;
                DialogResult::Continue
            }
            _ => DialogResult::Continue,
        }
    }

    fn sound(&self, _: bool) {}

    fn mainwin_cursor(&self) -> bool {
        self.mainwin_cursor
    }
}
