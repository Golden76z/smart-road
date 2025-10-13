use crate::config::{
    Broadcaster, Direction, Lane, MessageType, SPAWN_BOTTOM_EAST, SPAWN_BOTTOM_FORWARD,
    SPAWN_BOTTOM_WEST, SPAWN_LEFT_EAST, SPAWN_LEFT_FORWARD, SPAWN_LEFT_WEST, SPAWN_RIGHT_EAST,
    SPAWN_RIGHT_FORWARD, SPAWN_RIGHT_WEST, SPAWN_UP_EAST, SPAWN_UP_FORWARD, SPAWN_UP_WEST,
    SpawnManager,
};
use rand::prelude::*;

pub struct Vehicle {
    pub spawn: (i32, i32),
    pub lane: Lane,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(
        lane: Lane,
        direction: Direction,
        broadcast: &mut Broadcaster,
        spawn_manager: &mut SpawnManager,
    ) -> Option<Self> {
        let position: (i32, i32);

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

        // Checking if we can spawn the vehicle on the lane (Checking the cooldown)
        let can_spawn = spawn_manager.try_spawn(lane, direction);
        if can_spawn {
            // Creating the log notifiying the spawn
            let msg = format!("Spawn Vehicle on Lane: {:?} Going: {:?}", lane, direction);
            broadcast.log(&msg, MessageType::Info);

            Some(Vehicle {
                spawn: position,
                lane: lane,
                direction: direction,
            })
        } else {
            let msg = format!("Spawn in cooldown ! ({:?} going {:?})", lane, direction);
            broadcast.log(&msg, MessageType::Error);
            None
        }
    }

    pub fn spawn_random(
        lane: Lane,
        broadcaster: &mut Broadcaster,
        spawn_manager: &mut SpawnManager,
    ) {
        let mut rng = rand::rng();
        let rand_num = rng.random_range(0..3);
        match rand_num {
            0 => Vehicle::new(lane, Direction::West, broadcaster, spawn_manager),
            1 => Vehicle::new(lane, Direction::Forward, broadcaster, spawn_manager),
            2 => Vehicle::new(lane, Direction::East, broadcaster, spawn_manager),
            _ => unreachable!(),
        };
    }
}
