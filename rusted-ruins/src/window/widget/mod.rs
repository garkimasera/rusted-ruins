mod border;
mod gauge;
mod image;
mod label;
mod list;

use crate::context::*;
use crate::game::Command;

pub trait WidgetTrait {
    type Response;
    fn process_command(&mut self, _command: &Command) -> Option<Self::Response> {
        None
    }
    fn draw(&mut self, context: &mut Context);
}

pub use self::border::*;
pub use self::gauge::*;
pub use self::image::*;
pub use self::label::*;
pub use self::list::*;
