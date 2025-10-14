use crate::config::{
    DESTINATION_BOTTOM_EAST, DESTINATION_BOTTOM_FORWARD, DESTINATION_BOTTOM_WEST,
    DESTINATION_LEFT_EAST, DESTINATION_LEFT_FORWARD, DESTINATION_LEFT_WEST, DESTINATION_RIGHT_EAST,
    DESTINATION_RIGHT_FORWARD, DESTINATION_RIGHT_WEST, DESTINATION_UP_EAST, DESTINATION_UP_FORWARD,
    DESTINATION_UP_WEST, Direction, GameSettings, Lane, MessageType, SPAWN_BOTTOM_EAST,
    SPAWN_BOTTOM_FORWARD, SPAWN_BOTTOM_WEST, SPAWN_LEFT_EAST, SPAWN_LEFT_FORWARD, SPAWN_LEFT_WEST,
    SPAWN_RIGHT_EAST, SPAWN_RIGHT_FORWARD, SPAWN_RIGHT_WEST, SPAWN_UP_EAST, SPAWN_UP_FORWARD,
    SPAWN_UP_WEST, VELOCITY_NORMAL,
};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: i16,
    pub coordinates: (i32, i32),
    pub lane: Lane,
    pub direction: Direction,
    pub has_turned: bool,
    pub sprite_angle: f64,
    pub velocity: (i32, i32),
    pub color: u8,
}

impl Vehicle {
    pub fn new(lane: Lane, direction: Direction, config: &mut GameSettings) {
        // Checking if we can spawn the vehicle on the lane (Checking the cooldown)
        let can_spawn = config.spawn_manager.try_spawn(lane, direction);
        if can_spawn {
            // Generating a random number between 0 and 3 (for the sprite randomisation)
            let mut rng = rand::rng();
            let rand_num = rng.random_range(0..3);

            // Getting the spawn coordinates depending on the lane and direction
            let position: (i32, i32);
            let mut velocity: (i32, i32) = (0, 0);
            let sprite_angle: f64;
            match lane {
                Lane::Up => {
                    match direction {
                        Direction::West => position = SPAWN_UP_WEST,
                        Direction::Forward => position = SPAWN_UP_FORWARD,
                        Direction::East => position = SPAWN_UP_EAST,
                    };
                    velocity.1 = VELOCITY_NORMAL;
                    sprite_angle = 180.0;
                }
                Lane::Bottom => {
                    match direction {
                        Direction::West => position = SPAWN_BOTTOM_WEST,
                        Direction::Forward => position = SPAWN_BOTTOM_FORWARD,
                        Direction::East => position = SPAWN_BOTTOM_EAST,
                    };
                    velocity.1 = -VELOCITY_NORMAL;
                    sprite_angle = 0.0;
                }
                Lane::Left => {
                    match direction {
                        Direction::West => position = SPAWN_LEFT_WEST,
                        Direction::Forward => position = SPAWN_LEFT_FORWARD,
                        Direction::East => position = SPAWN_LEFT_EAST,
                    };
                    velocity.0 = VELOCITY_NORMAL;
                    sprite_angle = 90.0;
                }
                Lane::Right => {
                    match direction {
                        Direction::West => position = SPAWN_RIGHT_WEST,
                        Direction::Forward => position = SPAWN_RIGHT_FORWARD,
                        Direction::East => position = SPAWN_RIGHT_EAST,
                    };
                    velocity.0 = -VELOCITY_NORMAL;
                    sprite_angle = 270.0;
                }
            }

            // Creating the log notifiying the spawn
            let msg = format!("Spawn Vehicle on Lane: {:?} Going: {:?}", lane, direction);
            config.broadcaster.log(&msg, MessageType::Info);

            // Creating the new vehicle
            let vehicle = Vehicle {
                id: config.id(),
                coordinates: position,
                lane: lane,
                direction: direction,
                has_turned: false,
                sprite_angle: sprite_angle,
                velocity: velocity,
                color: rand_num,
            };

            // Inserting the vehicle in the TrafficLane struct
            config.lanes.insert_vehicle(lane, direction, vehicle);
        } else {
            let msg = format!("Spawn in cooldown ! ({:?} going {:?})", lane, direction);
            config.broadcaster.log(&msg, MessageType::Error);
        }
    }

    // Spawn a vehicle going to a random direction with a given lane
    pub fn spawn_random(lane: Lane, config: &mut GameSettings) {
        let mut rng = rand::rng();
        let rand_num = rng.random_range(0..3);
        match rand_num {
            0 => Vehicle::new(lane, Direction::West, config),
            1 => Vehicle::new(lane, Direction::Forward, config),
            2 => Vehicle::new(lane, Direction::East, config),
            _ => unreachable!(),
        };
    }

    pub fn update_position(&mut self, delta_time: f32) {
        self.coordinates.0 =
            (self.coordinates.0 as f32 + self.velocity.0 as f32 * delta_time) as i32;
        self.coordinates.1 =
            (self.coordinates.1 as f32 + self.velocity.1 as f32 * delta_time) as i32;
    }

    // Comparing coordinates with destination to check if the vehicle should turn or not
    pub fn should_turn(&mut self) -> bool {
        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => return self.coordinates.1 >= DESTINATION_UP_WEST.1,
                Direction::East => return self.coordinates.1 >= DESTINATION_UP_EAST.1,
                _ => false,
            },
            Lane::Bottom => match self.direction {
                Direction::West => return self.coordinates.1 <= DESTINATION_BOTTOM_WEST.1,
                Direction::East => return self.coordinates.1 <= DESTINATION_BOTTOM_EAST.1,
                _ => false,
            },
            Lane::Left => match self.direction {
                Direction::West => return self.coordinates.0 >= DESTINATION_LEFT_WEST.0,
                Direction::East => return self.coordinates.0 >= DESTINATION_LEFT_EAST.0,
                _ => false,
            },
            Lane::Right => match self.direction {
                Direction::West => return self.coordinates.0 <= DESTINATION_RIGHT_WEST.0,
                Direction::East => return self.coordinates.0 <= DESTINATION_RIGHT_EAST.0,
                _ => false,
            },
        }
    }

    // Method to change the velocity (turning left or right) & sprite angle
    pub fn turning(&mut self) {
        self.has_turned = true;

        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => {
                    self.velocity = (VELOCITY_NORMAL, 0);
                    self.sprite_angle = 90.0
                }
                Direction::East => {
                    self.velocity = (-VELOCITY_NORMAL, 0);
                    self.sprite_angle = 270.0
                }
                _ => {}
            },
            Lane::Bottom => match self.direction {
                Direction::West => {
                    self.velocity = (-VELOCITY_NORMAL, 0);
                    self.sprite_angle = 270.0
                }
                Direction::East => {
                    self.velocity = (VELOCITY_NORMAL, 0);
                    self.sprite_angle = 90.0
                }
                _ => {}
            },
            Lane::Left => match self.direction {
                Direction::West => {
                    self.velocity = (0, -VELOCITY_NORMAL);
                    self.sprite_angle = 0.0
                }
                Direction::East => {
                    self.velocity = (0, VELOCITY_NORMAL);
                    self.sprite_angle = 180.0
                }
                _ => {}
            },
            Lane::Right => match self.direction {
                Direction::West => {
                    self.velocity = (0, VELOCITY_NORMAL);
                    self.sprite_angle = 180.0
                }
                Direction::East => {
                    self.velocity = (0, -VELOCITY_NORMAL);
                    self.sprite_angle = 0.0
                }
                _ => {}
            },
        }
    }

    // Checking if a vehicle has reached its destination (should be removed from the array)
    pub fn has_reached_destination(&mut self) -> bool {
        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => {
                    self.coordinates.0 >= DESTINATION_UP_WEST.0
                        && self.coordinates.1 >= DESTINATION_UP_WEST.1
                }
                Direction::Forward => self.coordinates.1 >= DESTINATION_UP_FORWARD.1,
                Direction::East => {
                    self.coordinates.0 <= DESTINATION_UP_EAST.0
                        && self.coordinates.1 >= DESTINATION_UP_EAST.1
                }
            },
            Lane::Bottom => match self.direction {
                Direction::West => {
                    self.coordinates.0 <= DESTINATION_BOTTOM_WEST.0
                        && self.coordinates.1 <= DESTINATION_BOTTOM_WEST.1
                }
                Direction::Forward => self.coordinates.1 <= DESTINATION_BOTTOM_FORWARD.1,
                Direction::East => {
                    self.coordinates.0 >= DESTINATION_BOTTOM_EAST.0
                        && self.coordinates.1 <= DESTINATION_BOTTOM_EAST.1
                }
            },
            Lane::Left => match self.direction {
                Direction::West => {
                    self.coordinates.0 >= DESTINATION_LEFT_WEST.0
                        && self.coordinates.1 <= DESTINATION_LEFT_WEST.1
                }
                Direction::Forward => self.coordinates.0 >= DESTINATION_LEFT_FORWARD.0,
                Direction::East => {
                    self.coordinates.0 >= DESTINATION_LEFT_EAST.0
                        && self.coordinates.1 >= DESTINATION_LEFT_EAST.1
                }
            },
            Lane::Right => match self.direction {
                Direction::West => {
                    self.coordinates.0 <= DESTINATION_RIGHT_WEST.0
                        && self.coordinates.1 >= DESTINATION_RIGHT_WEST.1
                }
                Direction::Forward => self.coordinates.0 <= DESTINATION_RIGHT_FORWARD.0,
                Direction::East => {
                    self.coordinates.0 <= DESTINATION_RIGHT_EAST.0
                        && self.coordinates.1 <= DESTINATION_RIGHT_EAST.1
                }
            },
        }
    }
}
