use crate::{
    config::{
        ALMOST_STOP_HITBOX, BIG_HITBOX, Direction, MEDIUM_HITBOX, SMALL_HITBOX, STOP_HITBOX,
        TILE_HEIGHT, VEHICLE_HEIGHT, VEHICLE_WIDTH, VERY_SMALL_HITBOX,
    },
    simulation::Vehicle,
};
use sdl2::rect::Rect;

// Helper function to check if two rectangles intersect
pub fn rects_intersect(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.x() < rect2.x() + rect2.width() as i32
        && rect1.x() + rect1.width() as i32 > rect2.x()
        && rect1.y() < rect2.y() + rect2.height() as i32
        && rect1.y() + rect1.height() as i32 > rect2.y()
}

impl Vehicle {
    // Function to return a rectangle depending on the coordinates and rotation given
    pub fn rotated_rect(
        &mut self,
        coordinates: (i32, i32),
        offset: i32,
        rotation: f64,
    ) -> Vec<(Option<Rect>, Option<Rect>)> {
        let mut result: Vec<(Option<Rect>, Option<Rect>)> = Vec::new();
        let hitboxes: Vec<u32> = vec![
            BIG_HITBOX,
            MEDIUM_HITBOX,
            SMALL_HITBOX,
            VERY_SMALL_HITBOX,
            ALMOST_STOP_HITBOX,
            STOP_HITBOX,
        ];
        // println!("{:?}", rotation);

        // Creating all the possible hitboxes
        match rotation {
            0.0 => {
                for hitbox in hitboxes.iter() {
                    if *hitbox < offset as u32
                        || (self.direction == Direction::Forward || self.has_turned)
                    {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                *hitbox,
                                VEHICLE_HEIGHT,
                            )),
                            None,
                        ));
                    } else if self.direction == Direction::West {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                *hitbox - (*hitbox - offset as u32),
                                VEHICLE_HEIGHT,
                            )),
                            Some(Rect::new(
                                coordinates.0 + offset,
                                coordinates.1 - (*hitbox - TILE_HEIGHT) as i32 + offset,
                                VEHICLE_WIDTH,
                                *hitbox - offset as u32,
                            )),
                        ));
                    } else if self.direction == Direction::East {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                *hitbox - (*hitbox - offset as u32),
                                VEHICLE_HEIGHT,
                            )),
                            Some(Rect::new(
                                coordinates.0 + offset,
                                coordinates.1,
                                VEHICLE_WIDTH,
                                *hitbox - offset as u32,
                            )),
                        ));
                    }
                }
            }
            90.0 => {
                for hitbox in hitboxes.iter() {
                    if *hitbox < offset as u32
                        || (self.direction == Direction::Forward || self.has_turned)
                    {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                VEHICLE_WIDTH,
                                *hitbox,
                            )),
                            None,
                        ));
                    } else if self.direction == Direction::West {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                VEHICLE_WIDTH,
                                *hitbox - (*hitbox - offset as u32),
                            )),
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1 + *hitbox as i32 - (*hitbox as i32 - offset),
                                *hitbox - offset as u32,
                                VEHICLE_HEIGHT,
                            )),
                        ));
                    } else {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1,
                                VEHICLE_WIDTH,
                                *hitbox - (*hitbox - offset as u32),
                            )),
                            Some(Rect::new(
                                coordinates.0 - (*hitbox as i32 - offset) + VEHICLE_WIDTH as i32,
                                coordinates.1 + offset,
                                *hitbox - offset as u32,
                                VEHICLE_HEIGHT,
                            )),
                        ));
                    }
                }
            }
            180.0 => {
                for hitbox in hitboxes.iter() {
                    if *hitbox < offset as u32
                        || (self.direction == Direction::Forward || self.has_turned)
                    {
                        result.push((
                            Some(Rect::new(
                                coordinates.0 - (*hitbox - VEHICLE_WIDTH) as i32,
                                coordinates.1,
                                *hitbox,
                                VEHICLE_HEIGHT,
                            )),
                            None,
                        ));
                    } else if self.direction == Direction::West {
                        result.push((
                            Some(Rect::new(
                                coordinates.0 - (*hitbox - VEHICLE_WIDTH) as i32
                                    + (*hitbox as i32 - offset),
                                coordinates.1,
                                *hitbox - (*hitbox - offset as u32),
                                VEHICLE_HEIGHT,
                            )),
                            Some(Rect::new(
                                coordinates.0 - offset,
                                coordinates.1,
                                VEHICLE_WIDTH,
                                *hitbox - offset as u32,
                            )),
                        ));
                    } else {
                        result.push((
                            Some(Rect::new(
                                coordinates.0 - (*hitbox - VEHICLE_WIDTH) as i32
                                    + (*hitbox as i32 - offset),
                                coordinates.1,
                                *hitbox - (*hitbox - offset as u32),
                                VEHICLE_HEIGHT,
                            )),
                            Some(Rect::new(
                                coordinates.0 - *hitbox as i32 + (*hitbox as i32 - offset),
                                coordinates.1 - (*hitbox as i32 - offset) + VEHICLE_HEIGHT as i32,
                                VEHICLE_WIDTH,
                                *hitbox - offset as u32,
                            )),
                        ));
                    }
                }
            }
            270.0 => {
                for hitbox in hitboxes.iter() {
                    if *hitbox < offset as u32
                        || (self.direction == Direction::Forward || self.has_turned)
                    {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1 - (*hitbox - VEHICLE_HEIGHT) as i32,
                                VEHICLE_WIDTH,
                                *hitbox,
                            )),
                            None,
                        ));
                    } else if self.direction == Direction::West {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1 - (*hitbox - VEHICLE_HEIGHT) as i32
                                    + (*hitbox as i32 - offset),
                                VEHICLE_WIDTH,
                                *hitbox - (*hitbox - offset as u32),
                            )),
                            Some(Rect::new(
                                coordinates.0 + VEHICLE_WIDTH as i32 - (*hitbox as i32 - offset),
                                coordinates.1 - offset,
                                *hitbox - offset as u32,
                                VEHICLE_HEIGHT,
                            )),
                        ));
                    } else {
                        result.push((
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1 - (*hitbox - VEHICLE_HEIGHT) as i32
                                    + (*hitbox as i32 - offset),
                                VEHICLE_WIDTH,
                                *hitbox - (*hitbox - offset as u32),
                            )),
                            Some(Rect::new(
                                coordinates.0,
                                coordinates.1 - offset,
                                *hitbox - offset as u32,
                                VEHICLE_HEIGHT,
                            )),
                        ));
                    }
                }
            }
            _ => unreachable!(),
        };

        result
    }
}

// // Helper function to check intersection safely
// pub fn intersects_any(rect_opt: &Option<Rect>, other_hitbox1: &Rect, other_hitbox2: &Rect) -> bool {
//     if let Some(r) = rect_opt {
//         rects_intersect(r, other_hitbox1) || rects_intersect(r, other_hitbox2)
//     } else {
//         false
//     }
// }

pub fn intersects_any(
    rect_opt: &Option<Rect>,
    other_hitbox1: &Option<Rect>,
    other_hitbox2: &Option<Rect>,
) -> bool {
    if let Some(r) = rect_opt {
        if let Some(o1) = other_hitbox1 {
            if rects_intersect(r, o1) {
                return true;
            }
        }
        if let Some(o2) = other_hitbox2 {
            if rects_intersect(r, o2) {
                return true;
            }
        }
    }
    false
}
