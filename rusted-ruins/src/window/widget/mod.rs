
mod label;
mod icon;
mod list;

use game::Command;
use sdl2::render::WindowCanvas;
use sdlvalues::SdlValues;

pub trait WidgetTrait {
    type Response;
    fn process_command(&mut self, command: &Command) -> Option<Self::Response>;
    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues);
}

pub use self::label::*;
pub use self::icon::*;
pub use self::list::*;

