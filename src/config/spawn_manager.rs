use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::config::{Direction, Lane};

pub struct SpawnManager {
    // Key is (Lane, Direction), value is the last spawn time
    last_spawn: HashMap<(Lane, Direction), Instant>,
    cooldown: Duration,
}

impl SpawnManager {
    pub fn new(cooldown_ms: u64) -> Self {
        Self {
            last_spawn: HashMap::new(),
            cooldown: Duration::from_millis(cooldown_ms),
        }
    }

    pub fn can_spawn(&self, lane: Lane, direction: Direction) -> bool {
        match self.last_spawn.get(&(lane, direction)) {
            // If a key is found, check if the time elapsed is bigger than the cd
            Some(last_time) => last_time.elapsed() >= self.cooldown,
            None => true,
        }
    }

    pub fn record_spawn(&mut self, lane: Lane, direction: Direction) {
        self.last_spawn.insert((lane, direction), Instant::now());
    }

    pub fn try_spawn(&mut self, lane: Lane, direction: Direction) -> bool {
        if self.can_spawn(lane, direction) {
            self.record_spawn(lane, direction);
            true
        } else {
            false
        }
    }
}
