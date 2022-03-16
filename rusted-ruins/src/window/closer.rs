use super::commonuse::*;
use super::widget::{CloseButtonIconKind, CloseButtonWidget};
use crate::config::UI_CFG;
use sdl2::rect::Rect;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct DialogCloser {
    button: Option<CloseButtonWidget>,
    pub escape_click: bool,
}

pub fn set_on_newgame(b: bool) {
    ON_NEWGAME.store(b, Ordering::Relaxed);
}

static ON_NEWGAME: AtomicBool = AtomicBool::new(false);

impl DialogCloser {
    pub fn new(parent_rect: Rect) -> Self {
        let p = parent_rect.top_right();
        let icon_kind = if ON_NEWGAME.load(Ordering::Relaxed) {
            CloseButtonIconKind::Return
        } else {
            CloseButtonIconKind::Close
        };

        let button = CloseButtonWidget::from_bottom_right(
            p.x + UI_CFG.close_button_widget.closer_dx,
            p.y - UI_CFG.close_button_widget.closer_dy,
            icon_kind,
        );

        Self {
            button: Some(button),
            escape_click: false,
        }
    }

    pub fn process_command(&mut self, command: &Command) -> bool {
        if let Some(button) = &mut self.button {
            button.process_command(command) == Some(true)
        } else {
            false
        }
    }

    pub fn draw(&mut self, context: &mut Context<'_, '_, '_, '_>) {
        context.canvas.set_viewport(None);
        if let Some(button) = &mut self.button {
            button.draw(context);
        }
    }
}

macro_rules! closer {
    ($window:expr, $command:expr) => {
        closer!($window, $command, true)
    };
    ($window:expr, $command:expr, $reprocess:expr) => {{
        use crate::game::command::{Command, MouseButton};

        if $window.closer.process_command(&$command) {
            return DialogResult::Close;
        }

        match $command {
            Command::MouseButtonDown { x, y, button, .. } => {
                if *button == MouseButton::Left && !$window.rect.contains_point((*x, *y)) {
                    $window.closer.escape_click = true;
                }
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if $window.closer.escape_click {
                    $window.closer.escape_click = false;
                    if *button == MouseButton::Left && !$window.rect.contains_point((*x, *y)) {
                        if $reprocess {
                            return DialogResult::CloseAllAndReprocess($command.clone());
                        } else {
                            return DialogResult::Close;
                        }
                    }
                }
            }
            Command::MouseState {
                x,
                y,
                left_button,
                right_button,
                key_state,
                ..
            } => {
                if $reprocess {
                    if !$window.rect.contains_point((*x, *y)) {
                        let command = Command::MouseState {
                            x: *x,
                            y: *y,
                            left_button: *left_button,
                            right_button: *right_button,
                            key_state: *key_state,
                            ui_only: true,
                        };
                        return DialogResult::Reprocess(command);
                    }
                }
            }
            _ => (),
        }
    }};
}
