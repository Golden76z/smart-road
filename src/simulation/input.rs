use std::time::Instant;

use crate::{
    config::{ArrowKey, Direction, GameSettings, Lane, MessageType, Mode},
    simulation::Vehicle,
};
use sdl2::{event::Event, keyboard::Keycode};

impl<'a> GameSettings<'a> {
    pub fn input_listener(&mut self, event: Event) -> Result<(), String> {
        match event {
            // Listening for Key presses
            sdl2::event::Event::KeyDown {
                keycode: Some(k), ..
            } => {
                if k == Keycode::Escape {
                    return Err("Game stopped !".to_string());
                }
                if self.controller.pause {
                    match k {
                        Keycode::P => {
                            self.controller.pause_switch();
                            self.broadcaster.log("Game resumed", MessageType::Setting);
                            self.time_tracker = Instant::now();
                            Ok(())
                        }
                        _ => Ok(()),
                    }
                } else {
                    match k {
                        // Closing the programm when Escape key is pressed
                        Keycode::P => {
                            self.controller.pause_switch();
                            Ok(())
                        }

                        // Toggle the keybinds panel (ON by default)
                        Keycode::K => {
                            self.ui_state.toggle_keybinds();
                            let msg = format!(
                                "Keybinds panel switched from: {:?} to {:?}",
                                !self.ui_state.keybinds_panel, self.ui_state.keybinds_panel
                            );
                            self.broadcaster.log(&msg, MessageType::Setting);
                            Ok(())
                        }
                        // Toggle the statistic panel (ON by default)
                        Keycode::S => {
                            self.ui_state.toggle_statistic();
                            self.broadcaster
                                .log("Statistic panel switched", MessageType::Setting);
                            Ok(())
                        }
                        // Toggle the debug panel (OFF by default)
                        Keycode::D => {
                            self.ui_state.toggle_debug();
                            Ok(())
                        }

                        // Activating random mode
                        Keycode::R => {
                            match self.controller.mode {
                                Mode::Random => {
                                    let msg = format!("Random mode already on !");
                                    self.broadcaster.log(&msg, MessageType::Warning);
                                }
                                Mode::Manual => {
                                    let msg = format!("Mode switched from Manual to Random !");
                                    self.broadcaster.log(&msg, MessageType::Setting);
                                }
                            };
                            self.controller.random_mode();
                            Ok(())
                        }

                        // Activating manual mode
                        Keycode::M => {
                            match self.controller.mode {
                                Mode::Manual => {
                                    let msg = format!("Manual mode already on !");
                                    self.broadcaster.log(&msg, MessageType::Warning);
                                }
                                Mode::Random => {
                                    let msg = format!("Mode switched from Random to Manual.");
                                    self.broadcaster.log(&msg, MessageType::Setting);
                                }
                            };
                            self.controller.manual_mode();
                            Ok(())
                        }

                        _ => {
                            if self.controller.mode == Mode::Manual {
                                match k {
                                    Keycode::UP => {
                                        self.controller.set_arrow_key(ArrowKey::Up);
                                    }
                                    Keycode::DOWN => {
                                        self.controller.set_arrow_key(ArrowKey::Down);
                                    }
                                    Keycode::LEFT => {
                                        self.controller.set_arrow_key(ArrowKey::Left);
                                    }
                                    Keycode::RIGHT => {
                                        self.controller.set_arrow_key(ArrowKey::Right);
                                    }
                                    Keycode::Num1 => match self.controller.arrow_key {
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
                                    Keycode::Num2 => match self.controller.arrow_key {
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
                                    Keycode::Num3 => match self.controller.arrow_key {
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
                                    Keycode::Up => {
                                        Vehicle::spawn_random(Lane::Up, self);
                                        self.spawn_manager
                                            .record_key_pressed_random(Mode::Random, ArrowKey::Up);
                                    }
                                    Keycode::Down => {
                                        Vehicle::spawn_random(Lane::Bottom, self);
                                        self.spawn_manager.record_key_pressed_random(
                                            Mode::Random,
                                            ArrowKey::Down,
                                        );
                                    }
                                    Keycode::Left => {
                                        Vehicle::spawn_random(Lane::Left, self);
                                        self.spawn_manager.record_key_pressed_random(
                                            Mode::Random,
                                            ArrowKey::Left,
                                        );
                                    }
                                    Keycode::Right => {
                                        Vehicle::spawn_random(Lane::Right, self);
                                        self.spawn_manager.record_key_pressed_random(
                                            Mode::Random,
                                            ArrowKey::Right,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            Ok(())
                        }
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
