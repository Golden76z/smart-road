use sdl2::rect::Rect;

use crate::config::{GameSettings, MessageType};
use std::time::Instant;

impl<'a> GameSettings<'a> {
    pub fn update_position(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.time_tracker).as_secs_f32();
        self.time_tracker = now;

        let mut hitbox_vec: Vec<(i16, (Option<Rect>, Option<Rect>))> = Vec::new();

        // Ranging over the TrafficLane struct containing all the vehicles
        for ((_, _), vehicle_lane) in &self.lanes.lanes {
            // Lock the VecDeque<Vehicle>
            let mut queue = vehicle_lane.lock().unwrap();

            queue.retain_mut(|vehicle| {
                // Checking if the vehicles should turn left or right
                if vehicle.should_turn() && !vehicle.has_turned {
                    vehicle.turning();
                }

                // Checking if the vehicle goes out of bound
                let reached = vehicle.has_reached_destination();
                if reached {
                    let msg = format!("Vehicle: {:?} has reached destination !", vehicle.id);
                    self.broadcaster.log(&msg, MessageType::Info);
                }

                // Creating hitbox checking against already processed vehicles
                vehicle.create_hitbox(&hitbox_vec);

                // Adapt velocity depending on the hitbox
                vehicle.adapt_velocity();

                // Add this vehicle's hitbox to the processed list for next vehicles to check

                hitbox_vec.push((vehicle.id, (vehicle.hitbox.0, vehicle.hitbox.1)));
                // println!("Number of hitboxes: {:?}", hitbox_vec.len());

                // Updating position with vehicle velocity
                vehicle.update_position(delta_time);

                !reached
            });
        }
    }
}
