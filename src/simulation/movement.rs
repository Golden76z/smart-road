use crate::config::{Broadcaster, MessageType, TrafficLanes};
use std::time::Instant;

impl TrafficLanes {
    pub fn update_position(&mut self, time_tracker: &mut Instant, broadcaster: &mut Broadcaster) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(*time_tracker).as_secs_f32();
        *time_tracker = now;

        // Ranging over the TrafficLane struct containing all the vehicles
        for ((_, _), vehicle_lane) in &self.lanes {
            // Lock the VecDeque<Vehicle>
            let mut queue = vehicle_lane.lock().unwrap();

            queue.retain_mut(|vehicle| {
                // Updating position with vehicle velocity
                vehicle.update_position(delta_time);

                // Checking if the vehicles should turn left or right
                if vehicle.should_turn() && !vehicle.has_turned {
                    vehicle.turning();
                }

                // Checking if the vehicle goes out of bound
                let reached = vehicle.has_reached_destination();
                if reached {
                    let msg = format!("Vehicle: {:?} has reached destination !", vehicle.id);
                    broadcaster.log(&msg, MessageType::Info);
                }

                !reached
            });
        }
    }
}
