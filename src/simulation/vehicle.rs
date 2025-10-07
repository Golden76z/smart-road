use crate::config::{
    Broadcaster, Direction, Lane, MessageType, SPAWN_BOTTOM_EAST, SPAWN_BOTTOM_FORWARD,
    SPAWN_BOTTOM_WEST, SPAWN_LEFT_EAST, SPAWN_LEFT_FORWARD, SPAWN_LEFT_WEST, SPAWN_RIGHT_EAST,
    SPAWN_RIGHT_FORWARD, SPAWN_RIGHT_WEST, SPAWN_UP_EAST, SPAWN_UP_FORWARD, SPAWN_UP_WEST,
};
use rand::prelude::*;

pub struct Vehicle {
    pub spawn: (f32, f32),
    pub lane: Lane,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(lane: Lane, direction: Direction, broadcast: &mut Broadcaster) -> Self {
        let position: (f32, f32);

        match lane {
            Lane::Up => match direction {
                Direction::West => position = SPAWN_UP_WEST,
                Direction::Forward => position = SPAWN_UP_FORWARD,
                Direction::East => position = SPAWN_UP_EAST,
            },
            Lane::Bottom => match direction {
                Direction::West => position = SPAWN_BOTTOM_WEST,
                Direction::Forward => position = SPAWN_BOTTOM_FORWARD,
                Direction::East => position = SPAWN_BOTTOM_EAST,
            },
            Lane::Left => match direction {
                Direction::West => position = SPAWN_LEFT_WEST,
                Direction::Forward => position = SPAWN_LEFT_FORWARD,
                Direction::East => position = SPAWN_LEFT_EAST,
            },
            Lane::Right => match direction {
                Direction::West => position = SPAWN_RIGHT_WEST,
                Direction::Forward => position = SPAWN_RIGHT_FORWARD,
                Direction::East => position = SPAWN_RIGHT_EAST,
            },
        }

        let msg = format!("Spawn Vehicle on Lane: {:?} Going: {:?}", lane, direction);
        broadcast.log(&msg, MessageType::Info);

        Vehicle {
            spawn: position,
            lane: lane,
            direction: direction,
        }
    }

    pub fn spawn_random(lane: Lane, broadcaster: &mut Broadcaster) {
        let mut rng = rand::rng();
        let rand_num = rng.random_range(0..3);
        match rand_num {
            0 => Vehicle::new(lane, Direction::West, broadcaster),
            1 => Vehicle::new(lane, Direction::Forward, broadcaster),
            2 => Vehicle::new(lane, Direction::East, broadcaster),
            _ => unreachable!(),
        };
    }
}
