use sdl2::pixels::Color;
use sdl2::{image::LoadTexture, rect::Rect};
use std::time::Duration;

use crate::{
    config::{FPS, UiState},
    render::renderer::Renderer,
    simulation::input::input_listener,
};

mod config;
mod render;
mod simulation;

pub fn main() -> Result<(), String> {
    println!("<---- Road intersection start ---->");

    let mut ui = UiState {
        show_keybinds_panel: true,
        show_debug_panel: false,
    };

    let sdl_context = sdl2::init().unwrap();
    let mut render = Renderer::new(&sdl_context).expect("Failed to create a canvas");

    render.canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // renderer.canvas.clear();
        render.canvas.clear();

        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            match input_listener(event, &mut ui) {
                Ok(()) => {}
                Err(msg) => {
                    println!("{}", msg);
                    return Err("Error".to_string());
                }
            }
        }

        // Drawing the map textures
        render.create_map();

        // Render the drawn picture to the screen
        render.canvas.present();

        // Time between each loops - Frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
