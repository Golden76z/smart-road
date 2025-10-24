// <=============================== Screen settings ===============================>

pub const SCREEN_WIDTH: u32 = 1400;
pub const MAP_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 1000;
pub const FPS: u32 = 60;
pub const SPAWN_OFFSET_X: u32 = SCREEN_WIDTH / 20;
pub const SPAWN_OFFSET_Y: u32 = SCREEN_HEIGHT / 20;

// <============================= Broadcast settings ==============================>

pub const MAX_DEBUG_MESSAGES: u32 = 27;
// pub const BORDER_MARGIN: u32 = 15;
// pub const MESSAGE_SPAWN: u32 = SCREEN_WIDTH as u32 + 50;

// <================================ Grid settings ================================>

pub const TILE_WIDTH: u32 = MAP_WIDTH / 20;
pub const TILE_HEIGHT: u32 = SCREEN_HEIGHT / 20;

// <=============================== Physics settings ==============================>

pub const VEHICLE_WIDTH: u32 = TILE_WIDTH;
pub const VEHICLE_HEIGHT: u32 = TILE_HEIGHT;
pub const SAFE_DISTANCE: u32 = 3 * VEHICLE_WIDTH;

// Velocity - Pixels per second
pub const VELOCITY_ALMOST_STOP: i32 = 30;
pub const VELOCITY_VERY_SLOW: i32 = 60;
pub const VELOCITY_SLOW: i32 = 90;
pub const VELOCITY_NORMAL: i32 = 120;
pub const VELOCITY_FAST: i32 = 150;

// Hitboxes
pub const BIG_HITBOX: u32 = 300;
pub const MEDIUM_HITBOX: u32 = 225;
pub const SMALL_HITBOX: u32 = 150;
pub const VERY_SMALL_HITBOX: u32 = 75;
pub const ALMOST_STOP_HITBOX: u32 = 51;
pub const STOP_HITBOX: u32 = 50;

// Spawn cooldown
pub const SPAWN_COOLDOWN: u64 = 1200;
pub const KEY_COOLDOWN: u64 = 400;
pub const VELOCITY_COOLDOWN: u64 = 1;

// <=============================== Spawn positions ===============================>

// <----------------------------------  BOTTOM  ----------------------------------->

pub const SPAWN_BOTTOM_WEST: (i32, i32) = (
    (MAP_WIDTH / 2) as i32,
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32),
);
pub const SPAWN_BOTTOM_FORWARD: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 + TILE_WIDTH as i32,
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32),
);
pub const SPAWN_BOTTOM_EAST: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 + (2 * TILE_WIDTH as i32),
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32),
);

// <------------------------------------  UP  ------------------------------------->

pub const SPAWN_UP_WEST: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 - (TILE_WIDTH as i32),
    SPAWN_OFFSET_Y as i32 * -1,
);
pub const SPAWN_UP_FORWARD: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 - (2 * TILE_WIDTH as i32),
    SPAWN_OFFSET_Y as i32 * -1,
);
pub const SPAWN_UP_EAST: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 - (3 * TILE_WIDTH as i32),
    SPAWN_OFFSET_Y as i32 * -1,
);

// <-----------------------------------  LEFT  ------------------------------------>

pub const SPAWN_LEFT_WEST: (i32, i32) = (SPAWN_OFFSET_X as i32 * -1, (SCREEN_HEIGHT / 2) as i32);
pub const SPAWN_LEFT_FORWARD: (i32, i32) = (
    SPAWN_OFFSET_X as i32 * -1,
    (SCREEN_HEIGHT / 2) as i32 + TILE_HEIGHT as i32,
);
pub const SPAWN_LEFT_EAST: (i32, i32) = (
    SPAWN_OFFSET_X as i32 * -1,
    (SCREEN_HEIGHT / 2) as i32 + (2 * TILE_HEIGHT as i32),
);

// <-----------------------------------  RIGHT  ----------------------------------->

pub const SPAWN_RIGHT_WEST: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT / 2) as i32 - (TILE_HEIGHT as i32),
);
pub const SPAWN_RIGHT_FORWARD: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT / 2) as i32 - (2 * TILE_HEIGHT as i32),
);
pub const SPAWN_RIGHT_EAST: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT / 2) as i32 - (3 * TILE_HEIGHT as i32),
);

// <========================== Destinations coordinates ===========================>

// <----------------------------------  BOTTOM  ----------------------------------->

pub const DESTINATION_BOTTOM_WEST: (i32, i32) = (
    SPAWN_OFFSET_X as i32 * -1,
    ((SCREEN_HEIGHT / 2) - VEHICLE_HEIGHT) as i32,
);
pub const DESTINATION_BOTTOM_FORWARD: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 + VEHICLE_WIDTH as i32,
    SPAWN_OFFSET_Y as i32 * -1,
);
pub const DESTINATION_BOTTOM_EAST: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT / 2) as i32 + (2 * VEHICLE_HEIGHT as i32),
);

// <------------------------------------  UP  ------------------------------------->

pub const DESTINATION_UP_WEST: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT / 2) as i32,
);
pub const DESTINATION_UP_FORWARD: (i32, i32) = (
    (MAP_WIDTH / 2) as i32 - (2 * VEHICLE_HEIGHT as i32),
    (SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32,
);
pub const DESTINATION_UP_EAST: (i32, i32) = (
    SPAWN_OFFSET_X as i32 * -1,
    (SCREEN_HEIGHT as i32 / 2) - (3 * VEHICLE_HEIGHT as i32),
);

// <-----------------------------------  LEFT  ------------------------------------>

pub const DESTINATION_LEFT_WEST: (i32, i32) = (MAP_WIDTH as i32 / 2, SPAWN_OFFSET_Y as i32 * -1);
pub const DESTINATION_LEFT_FORWARD: (i32, i32) = (
    (MAP_WIDTH + SPAWN_OFFSET_X) as i32,
    (SCREEN_HEIGHT as i32 / 2) + VEHICLE_HEIGHT as i32,
);
pub const DESTINATION_LEFT_EAST: (i32, i32) = (
    (MAP_WIDTH as i32 / 2) - (3 * VEHICLE_WIDTH as i32),
    (SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32,
);

// <-----------------------------------  RIGHT  ----------------------------------->

pub const DESTINATION_RIGHT_WEST: (i32, i32) = (
    ((MAP_WIDTH / 2) - VEHICLE_WIDTH) as i32,
    (SCREEN_HEIGHT + SPAWN_OFFSET_Y) as i32,
);
pub const DESTINATION_RIGHT_FORWARD: (i32, i32) = (
    SPAWN_OFFSET_X as i32 * -1,
    (SCREEN_HEIGHT as i32 / 2) - (2 * VEHICLE_HEIGHT as i32),
);
pub const DESTINATION_RIGHT_EAST: (i32, i32) = (
    (MAP_WIDTH as i32 / 2) + (2 * VEHICLE_WIDTH as i32),
    SPAWN_OFFSET_Y as i32 * -1,
);
