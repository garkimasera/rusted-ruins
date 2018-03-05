
use sdl2::render::WindowCanvas;
use game::{Game, Animation};
use sdlvalues::SdlValues;
use window::Window;
use draw::mainwin::MainWinDrawer;
use config::SCREEN_CFG;

pub struct MainWindow {
    drawer: MainWinDrawer,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            drawer: MainWinDrawer::new(SCREEN_CFG.main_window.into()),
        }
    }

}

impl Window for MainWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {
        
        self.drawer.draw(canvas, game, sv, anim);
    }
}

