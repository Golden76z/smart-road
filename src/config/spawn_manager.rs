use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::config::{ArrowKey, Direction, Lane, Mode};

pub struct SpawnManager {
    // Key is (Lane, Direction), value is the last spawn time
    last_spawn: HashMap<(Lane, Direction), Instant>,
    last_key_pressed_random: HashMap<(Mode, ArrowKey), Instant>,
    last_key_pressed_manual: HashMap<Direction, Instant>,
    cooldown: Duration,
    key_cooldown: Duration,
}

impl SpawnManager {
    pub fn new(cooldown_ms: u64, key_cooldown: u64) -> Self {
        Self {
            last_spawn: HashMap::new(),
            last_key_pressed_random: HashMap::new(),
            last_key_pressed_manual: HashMap::new(),
            cooldown: Duration::from_millis(cooldown_ms),
            key_cooldown: Duration::from_millis(key_cooldown),
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

    pub fn record_key_pressed_random(&mut self, mode: Mode, key: ArrowKey) {
        self.last_key_pressed_random
            .insert((mode, key), Instant::now());
    }

    pub fn is_key_pressed_random(&mut self, mode: Mode, key: ArrowKey) -> bool {
        match self.last_key_pressed_random.get(&(mode, key)) {
            Some(last_time) => last_time.elapsed() <= self.key_cooldown,
            None => false,
        }
    }

    pub fn record_key_pressed_manual(&mut self, key: ArrowKey, direction: Direction) {
        if key != ArrowKey::None {
            self.last_key_pressed_manual
                .insert(direction, Instant::now());
        }
    }

    pub fn is_key_pressed_manual(&mut self, direction: Direction) -> bool {
        match self.last_key_pressed_manual.get(&direction) {
            Some(last_time) => last_time.elapsed() <= self.key_cooldown,
            None => false,
        }
    }
}
