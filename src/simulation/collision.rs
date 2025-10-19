use sdl2::rect::Rect;

use crate::{
    config::{Direction, HitboxType, Lane},
    simulation::{Vehicle, rects_intersect, rotated_rect},
};

impl Vehicle {
    // Method to create an hitbox for the vehicle
    pub fn create_hitbox(&mut self, hitboxes: &Vec<(i16, Rect)>) {
        let rotation: f64;

        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 0.0;
                    } else {
                        rotation = 90.0;
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
                    }
                }
            },
            Lane::Bottom => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 180.0;
                    } else {
                        rotation = 270.0;
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
                    }
                }
            },
            Lane::Left => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 270.0;
                    } else {
                        rotation = 0.0;
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
                    }
                }
            },
            Lane::Right => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        rotation = 90.0;
                    } else {
                        rotation = 180.0;
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
                    }
                }
            },
        }

        // Getting the vec of rect with different hitboxes depending on the angle
        let hitbox_vec: Vec<Rect> =
            rotated_rect((self.coordinates.0, self.coordinates.1), rotation);
        let mut selected_hitbox: Rect = hitbox_vec[0];

        // println!("{:?}", hitbox_vec);

        // Checking one by one which one will be assigned to the vehicle
        'hitbox_loop: for candidate_hitbox in &hitbox_vec {
            let mut has_collision = false;

            // Check against already processed vehicles
            for (other_id, other_hitbox) in hitboxes {
                // Skip checking against self (shouldn't happen, but just in case)
                if *other_id == self.id {
                    continue;
                }

                // Check if hitboxes overlap
                if rects_intersect(candidate_hitbox, other_hitbox) {
                    has_collision = true;
                    break;
                }
            }

            // If no collision found, use this hitbox
            if !has_collision {
                selected_hitbox = *candidate_hitbox;
                break 'hitbox_loop;
            }
        }

        // Find which index in hitbox_vec was selected
        match hitbox_vec.iter().position(|&h| h == selected_hitbox) {
            Some(0) => self.hitbox_type = HitboxType::Big,
            Some(1) => self.hitbox_type = HitboxType::Medium,
            Some(2) => self.hitbox_type = HitboxType::Small,
            Some(3) => self.hitbox_type = HitboxType::VerySmall,
            Some(4) => self.hitbox_type = HitboxType::Stop,
            _ => self.hitbox_type = HitboxType::Stop,
        }

        self.hitbox = Some(selected_hitbox);
    }
}
