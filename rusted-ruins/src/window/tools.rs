macro_rules! check_escape_click {
    ($window:expr, $command:expr) => {{
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
                        return DialogResult::Close;
                    }
                }
            }
            _ => (),
        }
    }};
}
