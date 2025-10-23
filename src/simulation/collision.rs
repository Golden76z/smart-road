use sdl2::rect::Rect;

use crate::{
    config::{
        DESTINATION_BOTTOM_EAST, DESTINATION_BOTTOM_WEST, DESTINATION_LEFT_EAST,
        DESTINATION_LEFT_WEST, DESTINATION_RIGHT_EAST, DESTINATION_RIGHT_WEST, DESTINATION_UP_EAST,
        DESTINATION_UP_WEST, Direction, HitboxType, Lane, SpawnManager,
    },
    simulation::{Vehicle, intersects_any},
};

impl Vehicle {
    // Method to create an hitbox for the vehicle
    pub fn create_hitbox(
        &mut self,
        spawn_manager: &mut SpawnManager,
        hitboxes: &Vec<(i16, (Option<Rect>, Option<Rect>))>,
    ) {
        let rotation: f64;
        let mut offset: i32 = 0;

        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 0.0;
                    } else {
                        rotation = 90.0;
                        offset = (DESTINATION_UP_WEST.1 - self.coordinates.1).abs();
                    }
                }
                Direction::Forward => {
                    rotation = 90.0;
                }
                Direction::East => {
                    if self.has_turned {
                        rotation = 180.0;
                    } else {
                        rotation = 90.0;
                        offset = (DESTINATION_UP_EAST.1 - self.coordinates.1).abs();
                    }
                }
            },
            Lane::Bottom => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 180.0;
                    } else {
                        rotation = 270.0;
                        offset = (DESTINATION_BOTTOM_WEST.1 - self.coordinates.1).abs();
                    }
                }
                Direction::Forward => {
                    rotation = 270.0;
                }
                Direction::East => {
                    if self.has_turned {
                        rotation = 0.0;
                    } else {
                        rotation = 270.0;
                        offset = (DESTINATION_BOTTOM_EAST.1 - self.coordinates.1).abs();
                    }
                }
            },
            Lane::Left => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 270.0;
                    } else {
                        rotation = 0.0;
                        offset = (DESTINATION_LEFT_WEST.0 - self.coordinates.0).abs();
                    }
                }
                Direction::Forward => {
                    rotation = 0.0;
                }
                Direction::East => {
                    if self.has_turned {
                        rotation = 90.0;
                    } else {
                        rotation = 0.0;
                        offset = (DESTINATION_LEFT_EAST.0 - self.coordinates.0).abs();
                    }
                }
            },
            Lane::Right => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 90.0;
                    } else {
                        rotation = 180.0;
                        offset = (DESTINATION_RIGHT_WEST.0 - self.coordinates.0).abs();
                    }
                }
                Direction::Forward => {
                    rotation = 180.0;
                }
                Direction::East => {
                    if self.has_turned {
                        rotation = 270.0;
                    } else {
                        rotation = 180.0;
                        offset = (DESTINATION_RIGHT_EAST.0 - self.coordinates.0).abs();
                    }
                }
            },
        }

        let can_change_velocity = spawn_manager.check_velocity_timer(self.id);

        // Generate all possible hitbox combinations based on rotation
        let hitbox_vec: Vec<(Option<Rect>, Option<Rect>)> =
            self.rotated_rect((self.coordinates.0, self.coordinates.1), offset, rotation);

        // Default to the first hitbox (fallback)
        let mut selected_hitbox = hitbox_vec[5];

        if can_change_velocity {
            // Try each possible hitbox
            'hitbox_loop: for (first, second) in &hitbox_vec {
                let mut has_collision = false;

                // Compare against all existing hitboxes
                for (other_id, (other_hitbox1, other_hitbox2)) in hitboxes {
                    if *other_id == self.id {
                        continue; // skip self
                    }

                    if intersects_any(first, other_hitbox1, other_hitbox2)
                        || intersects_any(second, other_hitbox1, other_hitbox2)
                    {
                        has_collision = true;
                        break;
                    }
                }

                // Use the first hitbox configuration that doesn’t collide
                if !has_collision {
                    selected_hitbox = (*first, *second);
                    break 'hitbox_loop;
                }
            }

            // Determine hitbox type based on index
            match hitbox_vec.iter().position(|&h| h == selected_hitbox) {
                Some(0) => self.hitbox_type = HitboxType::Big,
                Some(1) => self.hitbox_type = HitboxType::Medium,
                Some(2) => self.hitbox_type = HitboxType::Small,
                Some(3) => self.hitbox_type = HitboxType::VerySmall,
                Some(4) => self.hitbox_type = HitboxType::AlmostStop,
                Some(5) => self.hitbox_type = HitboxType::Stop,
                _ => self.hitbox_type = HitboxType::Stop,
            }
            spawn_manager.record_velocity(self.id);
        } else {
            match self.hitbox_type {
                HitboxType::Big => selected_hitbox = hitbox_vec[0],
                HitboxType::Medium => selected_hitbox = hitbox_vec[1],
                HitboxType::Small => selected_hitbox = hitbox_vec[2],
                HitboxType::VerySmall => selected_hitbox = hitbox_vec[3],
                HitboxType::AlmostStop => selected_hitbox = hitbox_vec[4],
                _ => selected_hitbox = hitbox_vec[5],
            }
        }

        // Assign final hitbox (still as Option<Rect>)
        self.hitbox = (selected_hitbox.0, selected_hitbox.1);
    }
}
