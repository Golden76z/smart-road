use crate::{
    config::{Direction, GameSettings, Lane},
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
                    let rand_num = rng.random_range(0..3);
                    match rand_num {
                        0 => Vehicle::new(Lane::Up, Direction::West),
                        1 => Vehicle::new(Lane::Up, Direction::Forward),
                        2 => Vehicle::new(Lane::Up, Direction::East),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                sdl2::keyboard::Keycode::Down => {
                    let rand_num = rng.random_range(0..3);
                    match rand_num {
                        0 => Vehicle::new(Lane::Bottom, Direction::West),
                        1 => Vehicle::new(Lane::Bottom, Direction::Forward),
                        2 => Vehicle::new(Lane::Bottom, Direction::East),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                sdl2::keyboard::Keycode::Left => {
                    let rand_num = rng.random_range(0..3);
                    match rand_num {
                        0 => Vehicle::new(Lane::Left, Direction::West),
                        1 => Vehicle::new(Lane::Left, Direction::Forward),
                        2 => Vehicle::new(Lane::Left, Direction::East),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                sdl2::keyboard::Keycode::Right => {
                    let rand_num = rng.random_range(0..3);
                    match rand_num {
                        0 => Vehicle::new(Lane::Right, Direction::West),
                        1 => Vehicle::new(Lane::Right, Direction::Forward),
                        2 => Vehicle::new(Lane::Right, Direction::East),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                sdl2::keyboard::Keycode::R => {
                    let rand_lane = rng.random_range(0..4);
                    match rand_lane {
                        0 => {
                            let rand_direction = rng.random_range(0..3);
                            match rand_direction {
                                0 => Vehicle::new(Lane::Up, Direction::West),
                                1 => Vehicle::new(Lane::Up, Direction::Forward),
                                2 => Vehicle::new(Lane::Up, Direction::East),
                                _ => unreachable!(),
                            };
                        }
                        1 => {
                            let rand_direction = rng.random_range(0..3);
                            match rand_direction {
                                0 => Vehicle::new(Lane::Bottom, Direction::West),
                                1 => Vehicle::new(Lane::Bottom, Direction::Forward),
                                2 => Vehicle::new(Lane::Bottom, Direction::East),
                                _ => unreachable!(),
                            };
                        }
                        2 => {
                            let rand_direction = rng.random_range(0..3);
                            match rand_direction {
                                0 => Vehicle::new(Lane::Left, Direction::West),
                                1 => Vehicle::new(Lane::Left, Direction::Forward),
                                2 => Vehicle::new(Lane::Left, Direction::East),
                                _ => unreachable!(),
                            };
                        }
                        3 => {
                            let rand_direction = rng.random_range(0..3);
                            match rand_direction {
                                0 => Vehicle::new(Lane::Right, Direction::West),
                                1 => Vehicle::new(Lane::Right, Direction::Forward),
                                2 => Vehicle::new(Lane::Right, Direction::East),
                                _ => unreachable!(),
                            };
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
