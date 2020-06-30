mod border;
mod button;
mod gauge;
mod image;
mod label;
mod list;
mod vscroll;

use crate::context::*;
use crate::game::Command;

pub trait WidgetTrait {
    type Response;
    fn process_command(&mut self, _command: &Command) -> Option<Self::Response> {
        None
    }
    fn draw(&mut self, context: &mut Context);
}

pub trait MovableWidget: WidgetTrait {
    fn move_to(&mut self, x: i32, y: i32);
}

pub use self::border::*;
pub use self::button::*;
pub use self::gauge::*;
pub use self::image::*;
pub use self::label::*;
pub use self::list::*;
pub use self::vscroll::*;
