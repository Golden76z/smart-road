use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::simulation::Vehicle;

pub enum Lane {
    Up,
    Bottom,
    Left,
    Right,
}

pub enum Direction {
    West,
    Forward,
    East,
}

pub type VehicleLane = Arc<Mutex<VecDeque<Vehicle>>>;

pub struct TrafficLanes {
    up: VehicleLane,
    bottom: VehicleLane,
    left: VehicleLane,
    right: VehicleLane,
}

impl TrafficLanes {
    pub fn new() -> Self {
        TrafficLanes {
            up: Arc::new(Mutex::new(VecDeque::new())),
            bottom: Arc::new(Mutex::new(VecDeque::new())),
            left: Arc::new(Mutex::new(VecDeque::new())),
            right: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}
