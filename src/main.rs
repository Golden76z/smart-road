use sdl2::pixels::Color;
use std::time::Duration;

use crate::{
    config::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH, UiState},
    simulation::input::input_listener,
};

mod config;
mod render;
mod simulation;

pub fn main() {
    println!("<---- Road intersection start ---->");

    let mut ui = UiState {
        show_keybinds_panel: true,
        show_debug_panel: false,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("smart_road", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // Test window
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));

    // Creating the new renderer - To print on the screen
    // let mut renderer = Renderer::new(window).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // renderer.canvas.clear();
        canvas.clear();

        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            match input_listener(event, &mut ui) {
                Ok(()) => {}
                Err(msg) => {
                    println!("{}", msg);
                    break 'running;
                }
            }
        }

        // Render the drawn picture to the screen
        // renderer.canvas.present();
        canvas.present();

        // Time between each loops - Frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
