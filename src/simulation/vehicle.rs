use crate::config::{
    Direction, Lane, SPAWN_BOTTOM_EAST, SPAWN_BOTTOM_FORWARD, SPAWN_BOTTOM_WEST, SPAWN_LEFT_EAST,
    SPAWN_LEFT_FORWARD, SPAWN_LEFT_WEST, SPAWN_RIGHT_EAST, SPAWN_RIGHT_FORWARD, SPAWN_RIGHT_WEST,
    SPAWN_UP_EAST, SPAWN_UP_FORWARD, SPAWN_UP_WEST,
};

pub struct Vehicle {
    pub spawn: (f32, f32),
    pub lane: Lane,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(lane: Lane, direction: Direction) -> Self {
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

        Vehicle {
            spawn: position,
            lane: lane,
            direction: direction,
        }
    }
}
