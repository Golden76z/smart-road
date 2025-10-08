use crate::config::GameSettings;
use sdl2::{image::LoadTexture, rect::Rect};

impl<'a> GameSettings<'a> {
    pub fn create_overlay(&mut self) {
        let texture_creator = self.render.canvas.texture_creator();

        let main_overlay = texture_creator
            .load_texture("../../assets/images/overlay/main_overlay.png")
            .unwrap();

        let statistic_overlay = texture_creator
            .load_texture("../../assets/images/overlay/statistic_overlay.png")
            .unwrap();

        let debug_overlay = texture_creator
            .load_texture("../../assets/images/overlay/debug_overlay.png")
            .unwrap();

        // Displaying the main overlay
        self.render
            .canvas
            .copy(&main_overlay, None, None)
            .expect("Error generating the main overlay");

        let coordinates_first = 2;
        let mut coordinates_second = 498;

        // Checking Ui State to see if keybinds overlay should be displayed
        if self.ui_state.keybinds_panel {
            self.keybinds_panel(&texture_creator, coordinates_first);
        }

        // Checking Ui State to see if statistic overlay should be displayed
        if self.ui_state.statistic_panel {
            // Checking if the statistic is the only panel displayed (displayed on top)
            if !self.ui_state.keybinds_panel {
                coordinates_second = coordinates_first;
            }
            self.render
                .canvas
                .copy(
                    &statistic_overlay,
                    None,
                    Rect::new(1000, coordinates_second, 400, 500),
                )
                .expect("Error generating the statistic overlay");
        }

        // If both Statistics & Keybinds overlay are off, display debug overlay
        if !self.ui_state.keybinds_panel && !self.ui_state.statistic_panel {
            self.render
                .canvas
                .copy(
                    &debug_overlay,
                    None,
                    Rect::new(1000, coordinates_first, 400, 1000),
                )
                .expect("Error generating the debug overlay");
        }
    }
}
