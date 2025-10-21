use crate::{
    config::{
        Direction, HitboxType, Lane, VELOCITY_ALMOST_STOP, VELOCITY_FAST, VELOCITY_NORMAL,
        VELOCITY_SLOW, VELOCITY_VERY_SLOW,
    },
    simulation::Vehicle,
};

impl Vehicle {
    // Method to adapt the velocity to the hitbox
    pub fn adapt_velocity(&mut self) {
        match self.lane {
            Lane::Up => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
                Direction::Forward => match self.hitbox_type {
                    HitboxType::Big => self.velocity = (0, VELOCITY_FAST),
                    HitboxType::Medium => self.velocity = (0, VELOCITY_NORMAL),
                    HitboxType::Small => self.velocity = (0, VELOCITY_SLOW),
                    HitboxType::VerySmall => self.velocity = (0, VELOCITY_VERY_SLOW),
                    HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                    HitboxType::Stop => self.velocity = (0, 0),
                },
                Direction::East => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (-VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (-VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (-VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (-VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (-VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
            },
            Lane::Bottom => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (-VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (-VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (-VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (-VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (-VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, -VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, -VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, -VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, -VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, -VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
                Direction::Forward => match self.hitbox_type {
                    HitboxType::Big => self.velocity = (0, -VELOCITY_FAST),
                    HitboxType::Medium => self.velocity = (0, -VELOCITY_NORMAL),
                    HitboxType::Small => self.velocity = (0, -VELOCITY_SLOW),
                    HitboxType::VerySmall => self.velocity = (0, -VELOCITY_VERY_SLOW),
                    HitboxType::AlmostStop => self.velocity = (0, -VELOCITY_ALMOST_STOP),
                    HitboxType::Stop => self.velocity = (0, 0),
                },
                Direction::East => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, -VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, -VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, -VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, -VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
            },
            Lane::Left => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, -VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, -VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, -VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, -VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, -VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
                Direction::Forward => match self.hitbox_type {
                    HitboxType::Big => self.velocity = (VELOCITY_FAST, 0),
                    HitboxType::Medium => self.velocity = (VELOCITY_NORMAL, 0),
                    HitboxType::Small => self.velocity = (VELOCITY_SLOW, 0),
                    HitboxType::VerySmall => self.velocity = (VELOCITY_VERY_SLOW, 0),
                    HitboxType::AlmostStop => self.velocity = (VELOCITY_ALMOST_STOP, 0),
                    HitboxType::Stop => self.velocity = (0, 0),
                },
                Direction::East => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
            },
            Lane::Right => match self.direction {
                Direction::West => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (-VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (-VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (-VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (-VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (-VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
                Direction::Forward => match self.hitbox_type {
                    HitboxType::Big => self.velocity = (-VELOCITY_FAST, 0),
                    HitboxType::Medium => self.velocity = (-VELOCITY_NORMAL, 0),
                    HitboxType::Small => self.velocity = (-VELOCITY_SLOW, 0),
                    HitboxType::VerySmall => self.velocity = (-VELOCITY_VERY_SLOW, 0),
                    HitboxType::AlmostStop => self.velocity = (-VELOCITY_ALMOST_STOP, 0),
                    HitboxType::Stop => self.velocity = (0, 0),
                },
                Direction::East => {
                    if self.has_turned {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (0, -VELOCITY_FAST),
                            HitboxType::Medium => self.velocity = (0, -VELOCITY_NORMAL),
                            HitboxType::Small => self.velocity = (0, -VELOCITY_SLOW),
                            HitboxType::VerySmall => self.velocity = (0, -VELOCITY_VERY_SLOW),
                            HitboxType::AlmostStop => self.velocity = (0, -VELOCITY_ALMOST_STOP),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    } else {
                        match self.hitbox_type {
                            HitboxType::Big => self.velocity = (-VELOCITY_FAST, 0),
                            HitboxType::Medium => self.velocity = (-VELOCITY_NORMAL, 0),
                            HitboxType::Small => self.velocity = (-VELOCITY_SLOW, 0),
                            HitboxType::VerySmall => self.velocity = (-VELOCITY_VERY_SLOW, 0),
                            HitboxType::AlmostStop => self.velocity = (-VELOCITY_ALMOST_STOP, 0),
                            HitboxType::Stop => self.velocity = (0, 0),
                        }
                    }
                }
            },
        }
    }
}
