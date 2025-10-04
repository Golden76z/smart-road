pub struct UiState {
    keybinds_panel: bool,
    statistic_panel: bool,
    debug_panel: bool,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            keybinds_panel: true,
            statistic_panel: true,
            debug_panel: false,
        }
    }

    fn toggle(value: &mut bool) {
        *value = !*value;
    }

    pub fn toggle_keybinds(&mut self) {
        Self::toggle(&mut self.keybinds_panel);
    }

    pub fn toggle_statistic(&mut self) {
        Self::toggle(&mut self.statistic_panel);
    }

    pub fn toggle_debug(&mut self) {
        Self::toggle(&mut self.debug_panel);
    }

    pub fn toggle_all(&mut self) {
        Self::toggle(&mut self.keybinds_panel);
        Self::toggle(&mut self.statistic_panel);
        Self::toggle(&mut self.debug_panel);
    }
}
