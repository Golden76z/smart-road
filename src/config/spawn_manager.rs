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
    last_velocity: HashMap<i16, Instant>,
    cooldown: Duration,
    key_cooldown: Duration,
    velocity_cooldown: Duration,
}

impl SpawnManager {
    pub fn new(cooldown_ms: u64, key_cooldown: u64, velocity_cooldown: u64) -> Self {
        Self {
            last_spawn: HashMap::new(),
            last_key_pressed_random: HashMap::new(),
            last_key_pressed_manual: HashMap::new(),
            last_velocity: HashMap::new(),
            cooldown: Duration::from_millis(cooldown_ms),
            key_cooldown: Duration::from_millis(key_cooldown),
            velocity_cooldown: Duration::from_millis(velocity_cooldown),
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

    // Inserting in the map the key pressed with the current time to be able to track
    pub fn record_key_pressed_random(&mut self, mode: Mode, key: ArrowKey) {
        self.last_key_pressed_random
            .insert((mode, key), Instant::now());
    }

    // Checking the cooldown of the keypress to check what key needs to be rendered
    pub fn is_key_pressed_random(&mut self, mode: Mode, key: ArrowKey) -> bool {
        match self.last_key_pressed_random.get(&(mode, key)) {
            Some(last_time) => last_time.elapsed() <= self.key_cooldown,
            None => false,
        }
    }

    // Inserting the manual key pressed in the map
    pub fn record_key_pressed_manual(&mut self, key: ArrowKey, direction: Direction) {
        if key != ArrowKey::None {
            self.last_key_pressed_manual
                .insert(direction, Instant::now());
        }
    }

    // Checking the time elapsed to check which key to render
    pub fn is_key_pressed_manual(&mut self, direction: Direction) -> bool {
        match self.last_key_pressed_manual.get(&direction) {
            Some(last_time) => last_time.elapsed() <= self.key_cooldown,
            None => false,
        }
    }

    // Method to insert in the map the vehicle with hitbox type to be able to track it
    pub fn record_velocity(&mut self, vehicle_id: i16) {
        self.last_velocity.insert(vehicle_id, Instant::now());
    }

    // Method to check if a vehicle can change his velocity
    pub fn check_velocity_timer(&mut self, vehicle_id: i16) -> bool {
        match self.last_velocity.get(&vehicle_id) {
            Some(last_time) => last_time.elapsed() >= self.velocity_cooldown,
            None => true,
        }
    }
}
