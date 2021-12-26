use crate::config::{CONFIG, SCREEN_CFG, UI_CFG};
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::eventhandler::EventHandler;
use crate::window::WindowManager;

pub struct Screen {
    canvas: WindowCanvas,
    event_handler: EventHandler,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Screen {
        let video_subsystem = sdl_context
            .video()
            .expect("Init Failed : SDL Video Subsystem");

        let (screen_w, screen_h) = if CONFIG.scale != 1 {
            info!("custom scale {}", CONFIG.scale);
            (
                (SCREEN_CFG.screen_w as i32 * CONFIG.scale) as u32,
                (SCREEN_CFG.screen_h as i32 * CONFIG.scale) as u32,
            )
        } else {
            (SCREEN_CFG.screen_w, SCREEN_CFG.screen_h)
        };

        let mut window = video_subsystem
            .window("Rusted Ruins", screen_w, screen_h)
            .position_centered()
            .build()
            .unwrap();
        window.set_icon(icon_surface());

        let canvas_builder = window.into_canvas();
        let canvas_builder = if CONFIG.hardware_acceleration {
            canvas_builder.accelerated()
        } else {
            canvas_builder.software()
        };
        let mut canvas = canvas_builder.build().unwrap();
        if CONFIG.scale != 1 {
            try_sdl!(canvas.set_scale(CONFIG.scale as f32, CONFIG.scale as f32));
        }

        Screen {
            canvas,
            event_handler: EventHandler::new(sdl_context),
        }
    }

    pub fn main_loop(&mut self, sdl_context: &crate::SdlContext, se: script::ScriptEngine) {
        let fps_duration = Duration::from_millis(1000 / 30);
        let mut event_pump = sdl_context.sdl_context.event_pump().unwrap();
        let mut prev_instant = Instant::now();
        let mut after_redraw_instant;
        let mut is_skip_next_frame = false;
        let texture_creator = self.canvas.texture_creator();
        let mut window_manager = WindowManager::new(sdl_context, &texture_creator, se);

        'mainloop: loop {
            self.event_handler.update(&event_pump);
            for event in event_pump.poll_iter() {
                if !self.event_handler.process_event(event) {
                    break 'mainloop;
                }
            }

            let mouse_state = event_pump.mouse_state();
            window_manager.update_cursor((
                mouse_state.x() / CONFIG.scale,
                mouse_state.y() / CONFIG.scale,
            ));
            if !window_manager.animation_now()
                && !window_manager.advance_turn(&mut self.event_handler)
            {
                break 'mainloop;
            }

            if !is_skip_next_frame {
                self.redraw(&mut window_manager);
            }

            after_redraw_instant = Instant::now();
            if after_redraw_instant > prev_instant + fps_duration {
                // Skip next drawing
                is_skip_next_frame = true;
            } else {
                let used_time = after_redraw_instant.duration_since(prev_instant);
                sleep(fps_duration - used_time);
                is_skip_next_frame = false;
            }
            prev_instant = Instant::now();
        }
    }

    fn redraw(&mut self, window_manager: &mut WindowManager<'_, '_>) {
        self.canvas.set_viewport(None);
        self.canvas.set_clip_rect(None);
        self.canvas.set_draw_color(UI_CFG.color.window_bg);
        if cfg!(target_os = "windows") {
            // Workaround for clear() in windows
            try_sdl!(self.canvas.fill_rect(sdl2::rect::Rect::new(
                0,
                0,
                SCREEN_CFG.screen_w,
                SCREEN_CFG.screen_h
            )));
        } else {
            self.canvas.clear();
        }
        window_manager.draw(&mut self.canvas);
        self.canvas.present();
    }
}

fn icon_surface() -> Surface<'static> {
    use sdl2::image::ImageRWops;
    use sdl2::rwops::RWops;

    let icon_png = include_bytes!("../../images/rusted-ruins_24x24.png");
    let rwops = RWops::from_bytes(icon_png).unwrap();
    rwops.load_png().expect("failed to load icon")
}
