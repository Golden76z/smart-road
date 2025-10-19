use crate::simulation::Vehicle;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Lane {
    Up,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    West,
    Forward,
    East,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HitboxType {
    Big,
    Medium,
    Small,
    VerySmall,
    Stop,
}

pub type VehicleLane = Arc<Mutex<VecDeque<Vehicle>>>;

pub struct TrafficLanes {
    pub lanes: HashMap<(Lane, Direction), VehicleLane>,
    // pub hitboxes: Vec<HitBox>,
}

impl TrafficLanes {
    pub fn new() -> Self {
        let mut lanes = HashMap::new();
        for lane in [Lane::Up, Lane::Bottom, Lane::Left, Lane::Right] {
            for dir in [Direction::West, Direction::Forward, Direction::East] {
                lanes.insert((lane, dir), Arc::new(Mutex::new(VecDeque::new())));
            }
        }
        Self {
            lanes,
            // hitboxes: Vec::new(),
        }
    }

    pub fn lane(&self, lane: Lane, dir: Direction) -> VehicleLane {
        self.lanes
            .get(&(lane, dir))
            .expect("Lane/Direction combination missing")
            .clone()
    }

    // Method to insert a new vehicle in the TrafficLane struct
    pub fn insert_vehicle(&mut self, lane: Lane, direction: Direction, vehicle: Vehicle) {
        let queue = self.lane(lane, direction);
        let mut q = queue.lock().unwrap();
        q.push_front(vehicle);
    }
}
