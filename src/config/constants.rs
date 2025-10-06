// <=============================== Screen settings ===============================>

pub const SCREEN_WIDTH: u32 = 1400;
pub const SCREEN_HEIGHT: u32 = 1000;
pub const FPS: u32 = 60;
pub const PAUSE: bool = false;
pub const SPAWN_OFFSET_X: u32 = SCREEN_WIDTH / 10;
pub const SPAWN_OFFSET_Y: u32 = SCREEN_HEIGHT / 10;

// <================================ Grid settings ================================>

pub const TILE_WIDTH: u32 = SCREEN_WIDTH / 20;
pub const TILE_HEIGHT: u32 = SCREEN_HEIGHT / 20;

// <=============================== Physics settings ==============================>

pub const VEHICLE_WIDTH: u32 = TILE_WIDTH;
pub const VEHICLE_HEIGHT: u32 = TILE_HEIGHT;
pub const SAFE_DISTANCE: u32 = 2 * VEHICLE_WIDTH;

// Velocity - Pixels per second
pub const VELOCITY_SLOW: u32 = 20;
pub const VELOCITY_NORMAL: u32 = 35;
pub const VELOCITY_FAST: u32 = 50;

// Spawn cooldown
pub const SPAWN_COOLDOWN: u64 = 1000;

// <=============================== Spawn positions ===============================>

// <----------------------------------  BOTTOM  ----------------------------------->

pub const SPAWN_BOTTOM_WEST: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (2.0 * TILE_WIDTH as f32) + (TILE_WIDTH as f32 / 2.0),
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as f32),
);
pub const SPAWN_BOTTOM_FORWARD: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (TILE_WIDTH as f32 + (TILE_WIDTH as f32 / 2.0)),
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as f32),
);
pub const SPAWN_BOTTOM_EAST: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (TILE_WIDTH as f32 / 2.0),
    ((SCREEN_HEIGHT + SPAWN_OFFSET_Y) as f32),
);

// <------------------------------------  UP  ------------------------------------->

pub const SPAWN_UP_WEST: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (TILE_WIDTH as f32 / 2.0),
    SPAWN_OFFSET_Y as f32 * -1.0,
);
pub const SPAWN_UP_FORWARD: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (TILE_WIDTH as f32 + (TILE_WIDTH as f32 / 2.0)),
    SPAWN_OFFSET_Y as f32 * -1.0,
);
pub const SPAWN_UP_EAST: (f32, f32) = (
    (SCREEN_WIDTH / 2) as f32 + (2.0 * TILE_WIDTH as f32) + (TILE_WIDTH as f32 / 2.0),
    SPAWN_OFFSET_Y as f32 * -1.0,
);

// <-----------------------------------  LEFT  ------------------------------------>

pub const SPAWN_LEFT_WEST: (f32, f32) = (
    SPAWN_OFFSET_X as f32 * -1.0,
    (SCREEN_HEIGHT / 2) as f32 + (TILE_HEIGHT as f32 / 2.0),
);
pub const SPAWN_LEFT_FORWARD: (f32, f32) = (
    SPAWN_OFFSET_X as f32 * -1.0,
    (SCREEN_HEIGHT / 2) as f32 + TILE_HEIGHT as f32 + (TILE_HEIGHT as f32 / 2.0),
);
pub const SPAWN_LEFT_EAST: (f32, f32) = (
    SPAWN_OFFSET_X as f32 * -1.0,
    (SCREEN_HEIGHT / 2) as f32 + (2.0 * TILE_HEIGHT as f32) + (TILE_HEIGHT as f32 / 2.0),
);

// <-----------------------------------  RIGHT  ----------------------------------->

pub const SPAWN_RIGHT_WEST: (f32, f32) = (
    (SCREEN_WIDTH + SPAWN_OFFSET_X) as f32,
    (SCREEN_HEIGHT / 2) as f32 + (TILE_HEIGHT as f32 / 2.0),
);
pub const SPAWN_RIGHT_FORWARD: (f32, f32) = (
    (SCREEN_WIDTH + SPAWN_OFFSET_X) as f32,
    (SCREEN_HEIGHT / 2) as f32 + TILE_HEIGHT as f32 + (TILE_HEIGHT as f32 / 2.0),
);
pub const SPAWN_RIGHT_EAST: (f32, f32) = (
    (SCREEN_WIDTH + SPAWN_OFFSET_X) as f32,
    (SCREEN_HEIGHT / 2) as f32 + (2.0 * TILE_HEIGHT as f32) + (TILE_HEIGHT as f32 / 2.0),
);
