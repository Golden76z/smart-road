use std::time::Duration;

mod config;
mod input;
mod render;
mod simulation;

use crate::{
    config::{CANVA_HEIGHT, CANVA_WIDTH},
    render::Renderer,
};
use input::input_listener;

pub fn main() {
    println!("<---- Road intersection start ---->");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    // Creating the new renderer - To print on the screen
    let mut renderer = Renderer::new(window).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        renderer.canvas.clear();

        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            // Keybinds events
        }

        // Render the drawn picture to the screen
        renderer.canvas.present();

        // Time between each loops - Frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}