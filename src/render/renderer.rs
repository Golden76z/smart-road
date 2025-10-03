use sdl2::{Sdl, render::WindowCanvas};

use crate::config::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("smart_road", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Renderer { canvas: canvas })
    }
}
