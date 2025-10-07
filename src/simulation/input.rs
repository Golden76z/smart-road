use crate::{
    config::{ArrowKey, Direction, GameSettings, Lane, MessageType},
    simulation::Vehicle,
};
use sdl2::event::Event;

impl<'a> GameSettings<'a> {
    pub fn input_listener(&mut self, event: Event) -> Result<(), String> {
        match event {
            // Listening for Key presses
            sdl2::event::Event::KeyDown {
                keycode: Some(k), ..
            } => {
                match k {
                    // Closing the programm when Escape key is pressed
                    sdl2::keyboard::Keycode::Escape => {
                        self.broadcaster.log("Game stop", MessageType::Info);
                        return Err("Program stopped".to_string());
                    }

                    // Toggle the keybinds panel (ON by default)
                    sdl2::keyboard::Keycode::I => {
                        self.ui_state.toggle_keybinds();
                        self.broadcaster
                            .log("Keybinds panel switched", MessageType::Info);
                        Ok(())
                    }
                    // Toggle the statistic panel (ON by default)
                    sdl2::keyboard::Keycode::O => {
                        self.ui_state.toggle_statistic();
                        self.broadcaster
                            .log("Statistic panel switched", MessageType::Setting);
                        Ok(())
                    }
                    // Toggle the debug panel (OFF by default)
                    sdl2::keyboard::Keycode::P => {
                        self.ui_state.toggle_debug();
                        Ok(())
                    }

                    // Activating random mode
                    sdl2::keyboard::Keycode::R => {
                        self.controller.random_mode();
                        self.broadcaster
                            .log("Random mode switched", MessageType::Error);
                        Ok(())
                    }

                    // Activating manual mode
                    sdl2::keyboard::Keycode::M => {
                        self.controller.manual_mode();
                        self.broadcaster
                            .log("Manual mode switched", MessageType::Warning);
                        Ok(())
                    }

                    _ => {
                        if self.controller.manual {
                            match k {
                                sdl2::keyboard::Keycode::UP => {
                                    self.controller.set_arrow_key(ArrowKey::Up);
                                }
                                sdl2::keyboard::Keycode::DOWN => {
                                    self.controller.set_arrow_key(ArrowKey::Down);
                                }
                                sdl2::keyboard::Keycode::LEFT => {
                                    self.controller.set_arrow_key(ArrowKey::Left);
                                }
                                sdl2::keyboard::Keycode::RIGHT => {
                                    self.controller.set_arrow_key(ArrowKey::Right);
                                }
                                sdl2::keyboard::Keycode::Num1 => match self.controller.arrow_key {
                                    ArrowKey::Up => {
                                        Vehicle::new(
                                            Lane::Up,
                                            Direction::West,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(
                                            Lane::Bottom,
                                            Direction::West,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(
                                            Lane::Left,
                                            Direction::West,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(
                                            Lane::Right,
                                            Direction::West,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                sdl2::keyboard::Keycode::Num2 => match self.controller.arrow_key {
                                    ArrowKey::Up => {
                                        Vehicle::new(
                                            Lane::Up,
                                            Direction::Forward,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(
                                            Lane::Bottom,
                                            Direction::Forward,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(
                                            Lane::Left,
                                            Direction::Forward,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(
                                            Lane::Right,
                                            Direction::Forward,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                sdl2::keyboard::Keycode::Num3 => match self.controller.arrow_key {
                                    ArrowKey::Up => {
                                        Vehicle::new(
                                            Lane::Up,
                                            Direction::East,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(
                                            Lane::Bottom,
                                            Direction::East,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(
                                            Lane::Left,
                                            Direction::East,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(
                                            Lane::Right,
                                            Direction::East,
                                            &mut self.broadcaster,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                _ => {}
                            }
                        } else {
                            match k {
                                sdl2::keyboard::Keycode::Up => {
                                    Vehicle::spawn_random(Lane::Up, &mut self.broadcaster);
                                }
                                sdl2::keyboard::Keycode::Down => {
                                    Vehicle::spawn_random(Lane::Bottom, &mut self.broadcaster);
                                }
                                sdl2::keyboard::Keycode::Left => {
                                    Vehicle::spawn_random(Lane::Left, &mut self.broadcaster);
                                }
                                sdl2::keyboard::Keycode::Right => {
                                    Vehicle::spawn_random(Lane::Right, &mut self.broadcaster);
                                }
                                _ => {}
                            }
                        }
                        Ok(())
                    }
                }
            }
            // Closing the programm when closing the window
            sdl2::event::Event::Quit { .. } => {
                return Err("Program stopped".to_string());
            }
            _ => Ok(()),
        }
    }
}
