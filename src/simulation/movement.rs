use crate::config::{GameSettings, MessageType};
use sdl2::rect::Rect;
use std::time::Instant;

impl<'a> GameSettings<'a> {
    pub fn update_position(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.time_tracker).as_secs_f32();
        self.time_tracker = now;

        let mut hitbox_vec: Vec<(i16, (Option<Rect>, Option<Rect>))> = Vec::new();

        // First pass: collect all hitboxes
        for ((_, _), vehicle_lane) in &self.lanes.lanes {
            let queue = vehicle_lane.lock().unwrap(); // read-only lock here
            for vehicle in queue.iter() {
                hitbox_vec.push((vehicle.id, vehicle.hitbox));
            }
        }

        // Second pass: update all vehicles
        for ((_, _), vehicle_lane) in &mut self.lanes.lanes {
            // Lock the VecDeque<Vehicle>
            let mut queue = vehicle_lane.lock().unwrap();

            // Collect vehicle IDs and coordinates before retain_mut to avoid borrow issues
            let vehicle_data: Vec<(i16, (i32, i32))> =
                queue.iter().map(|v| (v.id, v.coordinates)).collect();

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

                    // Statistics: increment the number of vehicles that passed through the intersection
                    self.statistics.record_vehicle_passed();

                    // Statistics: record time spent in the intersection for this vehicle
                    if let Some(stats) = self.statistics.vehicle_stats.get_mut(&vehicle.id) {
                        if stats.exit_time.is_none() {
                            stats.exit_time = Some(Instant::now());
                            let entry = stats.entry_time;
                            let time_in = entry.elapsed().as_secs_f32();
                            self.statistics.update_max_time_in_intersection(time_in);
                            self.statistics.update_min_time_in_intersection(time_in);
                        }
                    }
                }

                // Creating hitbox checking against already processed vehicles
                vehicle.create_hitbox(&mut self.spawn_manager, &hitbox_vec);

                // Adapt velocity depending on the hitbox
                vehicle.adapt_velocity();

                // Find the vehicle ahead's coordinates (if any)
                let current_idx = vehicle_data.iter().position(|(id, _)| *id == vehicle.id);

                let vehicle_ahead_coords = if let Some(idx) = current_idx {
                    if idx > 0 {
                        Some(vehicle_data[idx - 1].1) // coordinates of vehicle ahead
                    } else {
                        None // This is the first vehicle
                    }
                } else {
                    None
                };

                // Updating position with vehicle velocity
                vehicle.update_position(vehicle_ahead_coords, delta_time);

                // Statistics: record last position & scalar speed
                if let Some(stats) = self.statistics.vehicle_stats.get_mut(&vehicle.id) {
                    stats.positions.push(vehicle.coordinates);
                    let speed = ((vehicle.velocity.0.pow(2) + vehicle.velocity.1.pow(2)) as f32)
                        .sqrt() as i32;
                    stats.velocities.push(speed);
                    self.statistics.update_max_velocity(speed);
                    self.statistics.update_min_velocity(speed);
                }

                !reached
            });
        }
    }
}
