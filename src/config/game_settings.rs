use sdl2::{Sdl, ttf::Sdl2TtfContext};
use std::time::Instant;

use crate::{
    config::{
        Broadcaster, Controller, KEY_COOLDOWN, SPAWN_COOLDOWN, SpawnManager, TrafficLanes, UiState,
    },
    render::renderer::Renderer,
};

pub struct GameSettings<'a> {
    pub spawn_manager: SpawnManager,
    pub ui_state: UiState,
    pub render: Renderer,
    pub controller: Controller,
    pub broadcaster: Broadcaster<'a>,
    pub time_tracker: Instant,
    pub lanes: TrafficLanes,
    pub vehicle_id: i16,
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
            time_tracker: Instant::now(),
            lanes: TrafficLanes::new(),
            vehicle_id: 0,
        }
    }

    pub fn id(&mut self) -> i16 {
        self.vehicle_id += 1;
        self.vehicle_id
    }
}
