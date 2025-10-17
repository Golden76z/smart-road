use sdl2::rect::Rect;

use crate::config::{
    BIG_HITBOX, MEDIUM_HITBOX, SMALL_HITBOX, STOP_HITBOX, VEHICLE_HEIGHT, VEHICLE_WIDTH,
};

// Helper function to check if two rectangles intersect
pub fn rects_intersect(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.x() < rect2.x() + rect2.width() as i32
        && rect1.x() + rect1.width() as i32 > rect2.x()
        && rect1.y() < rect2.y() + rect2.height() as i32
        && rect1.y() + rect1.height() as i32 > rect2.y()
}

// Function to return a rectangle depending on the coordinates and rotation given
pub fn rotated_rect(coordinates: (i32, i32), rotation: f64) -> Vec<Rect> {
    // Creating all the possible hitboxes
    match rotation {
        0.0 => {
            vec![
                Rect::new(coordinates.0, coordinates.1, BIG_HITBOX, VEHICLE_HEIGHT),
                Rect::new(coordinates.0, coordinates.1, MEDIUM_HITBOX, VEHICLE_HEIGHT),
                Rect::new(coordinates.0, coordinates.1, SMALL_HITBOX, VEHICLE_HEIGHT),
                Rect::new(coordinates.0, coordinates.1, STOP_HITBOX, VEHICLE_HEIGHT),
            ]
        }
        90.0 => {
            vec![
                Rect::new(coordinates.0, coordinates.1, VEHICLE_WIDTH, BIG_HITBOX),
                Rect::new(coordinates.0, coordinates.1, VEHICLE_WIDTH, MEDIUM_HITBOX),
                Rect::new(coordinates.0, coordinates.1, VEHICLE_WIDTH, SMALL_HITBOX),
                Rect::new(coordinates.0, coordinates.1, VEHICLE_WIDTH, STOP_HITBOX),
            ]
        }
        180.0 => {
            vec![
                Rect::new(
                    coordinates.0 - (BIG_HITBOX - VEHICLE_WIDTH) as i32,
                    coordinates.1,
                    BIG_HITBOX,
                    VEHICLE_HEIGHT,
                ),
                Rect::new(
                    coordinates.0 - (MEDIUM_HITBOX - VEHICLE_WIDTH) as i32,
                    coordinates.1,
                    MEDIUM_HITBOX,
                    VEHICLE_HEIGHT,
                ),
                Rect::new(
                    coordinates.0 - (SMALL_HITBOX - VEHICLE_WIDTH) as i32,
                    coordinates.1,
                    SMALL_HITBOX,
                    VEHICLE_HEIGHT,
                ),
                Rect::new(
                    coordinates.0 - (STOP_HITBOX - VEHICLE_WIDTH) as i32,
                    coordinates.1,
                    STOP_HITBOX,
                    VEHICLE_HEIGHT,
                ),
            ]
        }
        270.0 => {
            vec![
                Rect::new(
                    coordinates.0,
                    coordinates.1 - (BIG_HITBOX - VEHICLE_HEIGHT) as i32,
                    VEHICLE_WIDTH,
                    BIG_HITBOX,
                ),
                Rect::new(
                    coordinates.0,
                    coordinates.1 - (MEDIUM_HITBOX - VEHICLE_HEIGHT) as i32,
                    VEHICLE_WIDTH,
                    MEDIUM_HITBOX,
                ),
                Rect::new(
                    coordinates.0,
                    coordinates.1 - (SMALL_HITBOX - VEHICLE_HEIGHT) as i32,
                    VEHICLE_WIDTH,
                    SMALL_HITBOX,
                ),
                Rect::new(
                    coordinates.0,
                    coordinates.1 - (STOP_HITBOX - VEHICLE_HEIGHT) as i32,
                    VEHICLE_WIDTH,
                    STOP_HITBOX,
                ),
            ]
        }
        _ => unreachable!(),
    }
}
