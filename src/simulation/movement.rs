use crate::config::TrafficLanes;
use std::time::Instant;

impl TrafficLanes {
    pub fn update_position(&mut self, time_tracker: &mut Instant) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(*time_tracker).as_secs_f32();
        *time_tracker = now;

        // Ranging over the TrafficLane struct containing all the vehicles
        for ((_, _), vehicle_lane) in &self.lanes {
            // Lock the VecDeque<Vehicle>
            let mut queue = vehicle_lane.lock().unwrap();

            // Iterate mutably over all vehicles in this lane
            for vehicle in queue.iter_mut() {
                vehicle.update_position(delta_time);
                if vehicle.should_turn() && !vehicle.has_turned {
                    vehicle.turning();
                }
            }
        }
    }
}
