use crate::config::{
    Direction, GameSettings, Lane, SPAWN_BOTTOM_EAST, SPAWN_BOTTOM_FORWARD, SPAWN_BOTTOM_WEST,
    SPAWN_LEFT_EAST, SPAWN_LEFT_FORWARD, SPAWN_LEFT_WEST, SPAWN_RIGHT_EAST, SPAWN_RIGHT_FORWARD,
    SPAWN_RIGHT_WEST, SPAWN_UP_EAST, SPAWN_UP_FORWARD, SPAWN_UP_WEST, VEHICLE_HEIGHT,
    VEHICLE_WIDTH,
};
use rand::prelude::*;
use sdl2::{image::LoadTexture, rect::Rect, render::TextureCreator, video::WindowContext};

impl<'a> GameSettings<'a> {
    pub fn render_vehicle(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        lane: Lane,
        direction: Direction,
    ) {
        // Generating a random number between 0 and 3 (for the sprite randomisation)
        let mut rng = rand::rng();
        let rand_num = rng.random_range(0..3);

        // Applying the sprite depending on the random number generated
        let vehicle;
        match rand_num {
            0 => {
                vehicle = texture_creator
                    .load_texture("../../assets/images/cars/car_1.png")
                    .unwrap();
            }
            1 => {
                vehicle = texture_creator
                    .load_texture("../../assets/images/cars/car_2.png")
                    .unwrap();
            }
            2 => {
                vehicle = texture_creator
                    .load_texture("../../assets/images/cars/car_3.png")
                    .unwrap();
            }
            3 => {
                vehicle = texture_creator
                    .load_texture("../../assets/images/cars/car_4.png")
                    .unwrap();
            }
            _ => unreachable!(),
        }

        match lane {
            Lane::Up => match direction {
                Direction::West => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_UP_WEST.0,
                            SPAWN_UP_WEST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::Forward => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_UP_FORWARD.0,
                            SPAWN_UP_FORWARD.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::East => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_UP_EAST.0,
                            SPAWN_UP_EAST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
            },
            Lane::Bottom => match direction {
                Direction::West => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_BOTTOM_WEST.0,
                            SPAWN_BOTTOM_WEST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::Forward => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_BOTTOM_FORWARD.0,
                            SPAWN_BOTTOM_FORWARD.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::East => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_BOTTOM_EAST.0,
                            SPAWN_BOTTOM_EAST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
            },
            Lane::Left => match direction {
                Direction::West => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_LEFT_WEST.0,
                            SPAWN_LEFT_WEST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::Forward => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_LEFT_FORWARD.0,
                            SPAWN_LEFT_FORWARD.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::East => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_LEFT_EAST.0,
                            SPAWN_LEFT_EAST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
            },
            Lane::Right => match direction {
                Direction::West => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_RIGHT_WEST.0,
                            SPAWN_RIGHT_WEST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::Forward => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_RIGHT_FORWARD.0,
                            SPAWN_RIGHT_FORWARD.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
                Direction::East => {
                    self.render.canvas.copy(
                        &vehicle,
                        None,
                        Rect::new(
                            SPAWN_RIGHT_EAST.0,
                            SPAWN_RIGHT_EAST.1,
                            VEHICLE_WIDTH,
                            VEHICLE_HEIGHT,
                        ),
                    );
                }
            },
        }
    }
}
