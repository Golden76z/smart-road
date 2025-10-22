use std::collections::{HashMap, HashSet};

use crate::config::SAFE_DISTANCE;

pub struct Statistics {
    pub vehicle_stats: HashMap<i16, VehicleStats>,
    pub vehicles_spawned: HashMap<(String, String), u32>,
    pub vehicles_passed: u32,
    pub max_velocity_reached: i32,
    pub min_velocity_reached: i32,
    pub max_time_in_intersection: f32,
    pub min_time_in_intersection: f32,
    pub close_calls: u32,
    pub close_call_pairs: HashSet<(i16, i16)>,
}

pub struct VehicleStats {
    pub entry_time: std::time::Instant,
    pub exit_time: Option<std::time::Instant>,
    pub positions: Vec<(i32, i32)>,
    pub velocities: Vec<i32>,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            vehicle_stats: HashMap::new(),
            vehicles_spawned: HashMap::new(),
            vehicles_passed: 0,
            max_velocity_reached: 0,
            min_velocity_reached: i32::MAX,
            max_time_in_intersection: 0.0,
            min_time_in_intersection: f32::MAX,
            close_calls: 0,
            close_call_pairs: HashSet::new(),
        }
    }

    /// Return a vector of strings representing key statistics for live display
    pub fn as_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push(format!("Nombre de véhicules passés : {}", self.vehicles_passed));
        lines.push(format!("Vitesse maximale atteinte : {}", self.max_velocity_reached));
        let min_v = if self.min_velocity_reached == i32::MAX { 0 } else { self.min_velocity_reached };
        lines.push(format!("Vitesse minimale atteinte : {}", min_v));
        lines.push(format!("Temps max dans l'intersection : {:.3} s", self.max_time_in_intersection));
        let min_time = if self.min_time_in_intersection == f32::MAX { 0.0 } else { self.min_time_in_intersection };
        lines.push(format!("Temps min dans l'intersection : {:.3} s", min_time));
        lines.push(format!("Nombre de close calls : {}", self.close_calls));
        lines
    }

    pub fn count_vehicle_spawned(&mut self, vehicle_type: &str, color: &str) {
        let key = (vehicle_type.to_string(), color.to_string());
        *self.vehicles_spawned.entry(key).or_insert(0) += 1;
    }

    pub fn record_vehicle_passed(&mut self) {
        self.vehicles_passed += 1;
    }

    pub fn update_max_velocity(&mut self, velocity: i32) {
        if velocity > self.max_velocity_reached {
            self.max_velocity_reached = velocity;
        }
    }

    pub fn update_min_velocity(&mut self, velocity: i32) {
        if velocity < self.min_velocity_reached {
            self.min_velocity_reached = velocity;
        }
    }

    pub fn update_max_time_in_intersection(&mut self, time: f32) {
        if time > self.max_time_in_intersection {
            self.max_time_in_intersection = time;
        }
    }

    pub fn update_min_time_in_intersection(&mut self, time: f32) {
        if time < self.min_time_in_intersection {
            self.min_time_in_intersection = time;
        }
    }

    pub fn record_close_call(&mut self) {
        self.close_calls += 1;
    }
    pub fn detect_close_calls(&mut self) {
        let vehicle_ids: Vec<i16> = self.vehicle_stats.keys().cloned().collect();
        for i in 0..vehicle_ids.len() {
            for j in (i + 1)..vehicle_ids.len() {
                let id1 = vehicle_ids[i];
                let id2 = vehicle_ids[j];
                let v1 = &self.vehicle_stats[&id1];
                let v2 = &self.vehicle_stats[&id2];
                if let (Some(&(x1, y1)), Some(&(x2, y2))) = (v1.positions.last(), v2.positions.last()) {
                    let dx = x1 - x2;
                    let dy = y1 - y2;
                    let dist2 = (dx * dx + dy * dy) as u32;
                    if dist2 < SAFE_DISTANCE * SAFE_DISTANCE {
                        let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };
                        if !self.close_call_pairs.contains(&pair) {
                            self.close_call_pairs.insert(pair);
                            self.record_close_call();
                        }
                    }
                }
            }
        }
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}