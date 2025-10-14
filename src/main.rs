use sdl2::ttf;
use std::time::Duration;

use crate::config::{FPS, MessageType};
use crate::config::{GameSettings, TrafficLanes};

mod config;
mod render;
mod simulation;

pub fn main() -> Result<(), String> {
    // Initializing the main config settings struct
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = ttf::init().unwrap();
    let mut game_config = GameSettings::new(&sdl_context, &ttf_context);

    let texture_creator = game_config.render.canvas.texture_creator();

    // Broadcasting the starting message
    game_config
        .broadcaster
        .log("Smart road start", config::MessageType::Info);

    // Initializing the structs needed
    let _lanes = TrafficLanes::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    '_running: loop {
        // renderer.canvas.clear();
        game_config.render.canvas.clear();

        // Creating the map textures
        game_config.render.create_map();

        // Input listener - Vehicle spawning & rendering
        for event in event_pump.poll_iter() {
            match game_config.input_listener(event) {
                Ok(()) => {}
                Err(msg) => {
                    game_config
                        .broadcaster
                        .log("Program stopped", MessageType::Error);
                    return Err(msg);
                }
            }
        }

        // Update the vehicles positions
        game_config
            .lanes
            .update_position(&mut game_config.time_tracker, &mut game_config.broadcaster);

        // Render the vehicles
        game_config.render_vehicles(&texture_creator);

        // Drawing the overlays
        game_config.create_overlay();

        // Writing the debug info on the panel (if visible)
        game_config.broadcaster.render(
            &mut game_config.render.canvas,
            &texture_creator,
            &mut game_config.ui_state,
        );

        // Render the drawn picture to the screen
        game_config.render.canvas.present();

        // Time between each loops - Frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
