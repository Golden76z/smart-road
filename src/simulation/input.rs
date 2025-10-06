use crate::{
    config::{GameSettings, Lane},
    simulation::Vehicle,
};
use rand::prelude::*;
use sdl2::event::Event;

impl GameSettings {
    pub fn input_listener(&mut self, event: Event) -> Result<(), String> {
        let mut rng = rand::rng();

        match event {
            // Listening for Key presses
            sdl2::event::Event::KeyDown {
                keycode: Some(k), ..
            } => match k {
                // Closing the programm when Escape key is pressed
                sdl2::keyboard::Keycode::Escape => {
                    return Err("Program stopped".to_string());
                }

                // Toggle the keybinds panel (ON by default)
                sdl2::keyboard::Keycode::I => {
                    self.ui_state.toggle_keybinds();
                    Ok(())
                }
                // Toggle the statistic panel (ON by default)
                sdl2::keyboard::Keycode::O => {
                    self.ui_state.toggle_statistic();
                    Ok(())
                }
                // Toggle the debug panel (OFF by default)
                sdl2::keyboard::Keycode::P => {
                    self.ui_state.toggle_debug();
                    Ok(())
                }

                sdl2::keyboard::Keycode::Up => {
                    Vehicle::spawn_random(Lane::Up);
                    Ok(())
                }
                sdl2::keyboard::Keycode::Down => {
                    Vehicle::spawn_random(Lane::Bottom);
                    Ok(())
                }
                sdl2::keyboard::Keycode::Left => {
                    Vehicle::spawn_random(Lane::Left);
                    Ok(())
                }
                sdl2::keyboard::Keycode::Right => {
                    Vehicle::spawn_random(Lane::Right);
                    Ok(())
                }
                sdl2::keyboard::Keycode::R => {
                    let rand_lane = rng.random_range(0..4);
                    match rand_lane {
                        0 => {
                            Vehicle::spawn_random(Lane::Up);
                        }
                        1 => {
                            Vehicle::spawn_random(Lane::Bottom);
                        }
                        2 => {
                            Vehicle::spawn_random(Lane::Left);
                        }
                        3 => {
                            Vehicle::spawn_random(Lane::Right);
                        }
                        _ => unreachable!(),
                    };
                    Ok(())
                }

                _ => Ok(()),
            },
            // Closing the programm when closing the window
            sdl2::event::Event::Quit { .. } => {
                return Err("Program stopped".to_string());
            }
            _ => Ok(()),
        }
    }
}
