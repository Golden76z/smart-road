use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::collections::HashMap;

pub struct Textures<'a> {
    pub vehicle: Texture<'a>,
    pub keys: Texture<'a>,
    pub road: HashMap<&'static str, Texture<'a>>,
    pub overlay: HashMap<&'static str, Texture<'a>>,
}

impl<'a> Textures<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        // Load keys sprite
        let keys = texture_creator
            .load_texture("../../assets/images/keys.png")
            .map_err(|e| format!("Failed to load keys texture: {}", e))?;

        // Load vehicle sprites
        let vehicle = texture_creator
            .load_texture("../../assets/images/car_sprites.png")
            .map_err(|e| format!("Failed to load vehicle texture: {}", e))?;

        // Load road textures into a HashMap
        let mut road = HashMap::new();
        road.insert(
            "Vertical",
            texture_creator
                .load_texture("../../assets/images/road/lane_vertical.png")
                .map_err(|e| format!("Failed to load Vertical road texture: {}", e))?,
        );
        road.insert(
            "Horizontal",
            texture_creator
                .load_texture("../../assets/images/road/lane_horizontal.png")
                .map_err(|e| format!("Failed to load Horizontal road texture: {}", e))?,
        );
        road.insert(
            "Corner",
            texture_creator
                .load_texture("../../assets/images/road/bottom-left.png")
                .map_err(|e| format!("Failed to load Corner road texture: {}", e))?,
        );
        road.insert(
            "Center",
            texture_creator
                .load_texture("../../assets/images/road/center.png")
                .map_err(|e| format!("Failed to load Center road texture: {}", e))?,
        );

        // Loading the overlay and storing them into a HashMap
        let mut overlay = HashMap::new();
        overlay.insert(
            "Main",
            texture_creator
                .load_texture("../../assets/images/overlay/main_overlay.png")
                .map_err(|e| format!("Failed to load the Main overlay: {}", e))?,
        );
        overlay.insert(
            "Keybinds",
            texture_creator
                .load_texture("../../assets/images/overlay/setting_overlay.png")
                .map_err(|e| format!("Failed to load the Keybinds overlay: {}", e))?,
        );
        overlay.insert(
            "Stats",
            texture_creator
                .load_texture("../../assets/images/overlay/statistic_overlay.png")
                .map_err(|e| format!("Failed to load the Statistics overlay: {}", e))?,
        );
        overlay.insert(
            "Debug",
            texture_creator
                .load_texture("../../assets/images/overlay/debug_overlay.png")
                .map_err(|e| format!("Failed to load the Debug overlay: {}", e))?,
        );
        overlay.insert(
            "Manual",
            texture_creator
                .load_texture("../../assets/images/overlay/manual_keybinds_overlay.png")
                .map_err(|e| format!("Failed to load the Manual Keybinds overlay: {}", e))?,
        );
        overlay.insert(
            "Random",
            texture_creator
                .load_texture("../../assets/images/overlay/random_keybinds_overlay.png")
                .map_err(|e| format!("Failed to load the Random Keybinds overlay: {}", e))?,
        );
        overlay.insert(
            "Pause",
            texture_creator
                .load_texture("../../assets/images/overlay/pause_overlay.png")
                .map_err(|e| format!("Failed to load the Pause overlay: {}", e))?,
        );

        Ok(Self {
            vehicle,
            keys,
            road,
            overlay,
        })
    }
}
