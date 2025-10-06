use sdl2::{image::LoadTexture, rect::Rect};

use crate::{config::UiState, render::renderer::Renderer};

impl Renderer {
    pub fn create_overlay(&mut self, ui_state: &mut UiState) {
        let texture_creator = self.canvas.texture_creator();

        let main_overlay = texture_creator
            .load_texture("../../assets/main_overlay.png")
            .unwrap();

        let keybinds_overlay = texture_creator
            .load_texture("../../assets/setting_overlay.png")
            .unwrap();

        let statistic_overlay = texture_creator
            .load_texture("../../assets/statistic_overlay.png")
            .unwrap();

        // Displaying the main overlay
        self.canvas
            .copy(&main_overlay, None, None)
            .expect("Error generating the main overlay");

        let coordinates_first = 2;
        let mut coordinates_second = 498;

        // Checking Ui State to see if keybinds overlay should be displayed
        if ui_state.keybinds_panel {
            self.canvas
                .copy(
                    &keybinds_overlay,
                    None,
                    Rect::new(1000, coordinates_first, 400, 500),
                )
                .expect("Error generating the keybinds overlay");
        }

        // Checking Ui State to see if statistic overlay should be displayed
        if ui_state.statistic_panel {
            // Checking if the statistic is the only panel displayed (displayed on top)
            if !ui_state.keybinds_panel {
                coordinates_second = coordinates_first;
            }
            self.canvas
                .copy(
                    &statistic_overlay,
                    None,
                    Rect::new(1000, coordinates_second, 400, 500),
                )
                .expect("Error generating the statistic overlay");
        }
    }
}
