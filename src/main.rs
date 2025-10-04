use std::time::Duration;

use crate::config::{GameSettings, TrafficLanes};
// use crate::simulation::Vehicle;
use crate::{config::FPS, render::renderer::Renderer};

mod config;
mod render;
mod simulation;

pub fn main() -> Result<(), String> {
    println!("<---- Road intersection start ---->");

    // Initializing the main config settings struct
    let mut game_config = GameSettings::new();

    // Initializing the structs needed
    let sdl_context = sdl2::init().unwrap();
    let mut render = Renderer::new(&sdl_context).expect("Failed to create a canvas");
    let _lanes = TrafficLanes::new();

    // render.canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();

    '_running: loop {
        // renderer.canvas.clear();
        render.canvas.clear();

        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            match game_config.input_listener(event) {
                Ok(()) => {}
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        // Vehicle::new(config::Lane::Up, Direction::West);

        // Drawing the map textures
        render.create_map();

        // Render the drawn picture to the screen
        render.canvas.present();

        // Time between each loops - Frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
