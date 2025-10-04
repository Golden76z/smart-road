use crate::config::{SPAWN_COOLDOWN, SpawnManager, UiState};

pub struct GameSettings {
    pub spawn_manager: SpawnManager,
    pub ui_state: UiState,
}

impl GameSettings {
    pub fn new() -> Self {
        GameSettings {
            spawn_manager: SpawnManager::new(SPAWN_COOLDOWN),
            ui_state: UiState::new(),
        }
    }
}
