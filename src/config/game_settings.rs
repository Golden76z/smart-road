use sdl2::{Sdl, ttf::Sdl2TtfContext};

use crate::{
    config::{Broadcaster, Controller, KEY_COOLDOWN, SPAWN_COOLDOWN, SpawnManager, UiState},
    render::renderer::Renderer,
};

pub struct GameSettings<'a> {
    pub spawn_manager: SpawnManager,
    pub ui_state: UiState,
    pub render: Renderer,
    pub controller: Controller,
    pub broadcaster: Broadcaster<'a>,
}

impl<'a> GameSettings<'a> {
    pub fn new(sdl_context: &Sdl, ttf_context: &'a Sdl2TtfContext) -> Self {
        let render = Renderer::new(&sdl_context).expect("Failed to create a renderer");

        GameSettings {
            spawn_manager: SpawnManager::new(SPAWN_COOLDOWN, KEY_COOLDOWN),
            ui_state: UiState::new(),
            render: render,
            controller: Controller::new(),
            broadcaster: Broadcaster::new(ttf_context),
        }
    }
}
