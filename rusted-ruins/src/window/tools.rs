macro_rules! check_escape_click {
    ($window:expr, $command:expr) => {
        check_escape_click!($window, $command, true)
    };
    ($window:expr, $command:expr, $reprocess:expr) => {{
        use crate::game::command::{Command, MouseButton};
        match $command {
            Command::MouseButtonDown { x, y, button, .. } => {
                if *button == MouseButton::Left && !$window.rect.contains_point((*x, *y)) {
                    $window.escape_click = true;
                }
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if $window.escape_click {
                    $window.escape_click = false;
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
