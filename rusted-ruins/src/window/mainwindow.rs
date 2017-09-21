
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use game::{Game, Animation};
use sdlvalues::SdlValues;
use window::Window;
use draw::mainwin::MainWinDrawer;

pub struct MainWindow {
    rect: Rect,
    drawer: MainWinDrawer,
}

impl MainWindow {
    pub fn new(rect: Rect) -> MainWindow {
        MainWindow {
            rect: rect,
            drawer: MainWinDrawer::new(rect),
        }
    }

}

impl Window for MainWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {
        
        self.drawer.draw(canvas, game, sv, anim);
    }
}

