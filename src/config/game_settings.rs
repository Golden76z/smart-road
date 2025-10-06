use sdl2::Sdl;

use crate::{
    config::{SPAWN_COOLDOWN, SpawnManager, UiState},
    render::renderer::Renderer,
};

pub struct GameSettings {
    pub spawn_manager: SpawnManager,
    pub ui_state: UiState,
    pub render: Renderer,
}

impl GameSettings {
    pub fn new(sdl_context: &Sdl) -> Self {
        let render = Renderer::new(&sdl_context).expect("Failed to create a renderer");

        GameSettings {
            spawn_manager: SpawnManager::new(SPAWN_COOLDOWN),
            ui_state: UiState::new(),
            render: render,
        }
    }
}
