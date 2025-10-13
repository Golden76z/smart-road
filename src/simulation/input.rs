use crate::{
    config::{ArrowKey, Direction, GameSettings, Lane, MessageType, Mode},
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
                        self.broadcaster.log("Game stop", MessageType::Setting);
                        return Err("Program stopped".to_string());
                    }

                    // Toggle the keybinds panel (ON by default)
                    sdl2::keyboard::Keycode::I => {
                        self.ui_state.toggle_keybinds();
                        let msg = format!(
                            "Keybinds panel switched from: {:?} to {:?}",
                            !self.ui_state.keybinds_panel, self.ui_state.keybinds_panel
                        );
                        self.broadcaster.log(&msg, MessageType::Setting);
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
                            .log("Random mode switched", MessageType::Warning);
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
                                        Vehicle::new(Lane::Up, Direction::West, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Up,
                                            Direction::West,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(Lane::Bottom, Direction::West, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Down,
                                            Direction::West,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(Lane::Left, Direction::West, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Left,
                                            Direction::West,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(Lane::Right, Direction::West, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Right,
                                            Direction::West,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                sdl2::keyboard::Keycode::Num2 => match self.controller.arrow_key {
                                    ArrowKey::Up => {
                                        Vehicle::new(Lane::Up, Direction::Forward, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Up,
                                            Direction::Forward,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(Lane::Bottom, Direction::Forward, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Down,
                                            Direction::Forward,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(Lane::Left, Direction::Forward, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Left,
                                            Direction::Forward,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(Lane::Right, Direction::Forward, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Right,
                                            Direction::Forward,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                sdl2::keyboard::Keycode::Num3 => match self.controller.arrow_key {
                                    ArrowKey::Up => {
                                        Vehicle::new(Lane::Up, Direction::East, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Up,
                                            Direction::East,
                                        );
                                    }
                                    ArrowKey::Down => {
                                        Vehicle::new(Lane::Bottom, Direction::East, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Down,
                                            Direction::East,
                                        );
                                    }
                                    ArrowKey::Left => {
                                        Vehicle::new(Lane::Left, Direction::East, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Left,
                                            Direction::East,
                                        );
                                    }
                                    ArrowKey::Right => {
                                        Vehicle::new(Lane::Right, Direction::East, self);
                                        self.spawn_manager.record_key_pressed_manual(
                                            ArrowKey::Right,
                                            Direction::East,
                                        );
                                    }
                                    ArrowKey::None => {}
                                },
                                _ => {}
                            }
                        } else {
                            match k {
                                sdl2::keyboard::Keycode::Up => {
                                    Vehicle::spawn_random(Lane::Up, self);
                                    self.spawn_manager
                                        .record_key_pressed_random(Mode::random, ArrowKey::Up);
                                }
                                sdl2::keyboard::Keycode::Down => {
                                    Vehicle::spawn_random(Lane::Bottom, self);
                                    self.spawn_manager
                                        .record_key_pressed_random(Mode::random, ArrowKey::Down);
                                }
                                sdl2::keyboard::Keycode::Left => {
                                    Vehicle::spawn_random(Lane::Left, self);
                                    self.spawn_manager
                                        .record_key_pressed_random(Mode::random, ArrowKey::Left);
                                }
                                sdl2::keyboard::Keycode::Right => {
                                    Vehicle::spawn_random(Lane::Right, self);
                                    self.spawn_manager
                                        .record_key_pressed_random(Mode::random, ArrowKey::Right);
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
