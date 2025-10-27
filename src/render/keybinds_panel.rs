use crate::{
    config::{ArrowKey, Direction, GameSettings, Mode},
    render::textures::Textures,
};
use sdl2::rect::Rect;

impl<'a> GameSettings<'a> {
    pub fn basic_keys(&mut self, textures: &Textures) {
        let keys = &textures.keys;

        // Rendering the ui keys (Keybinds - Stats - Debug)
        if self.ui_state.keybinds_panel {
            self.render
                .canvas
                .copy(&keys, Rect::new(300, 50, 50, 50), Rect::new(39, 41, 50, 50))
                .expect("Failed to load the Keybinds down key")
        } else {
            self.render
                .canvas
                .copy(&keys, Rect::new(300, 0, 50, 50), Rect::new(39, 41, 50, 50))
                .expect("Failed to load the Keybinds up key")
        }
        if self.ui_state.statistic_panel {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(350, 50, 50, 50),
                    Rect::new(140, 41, 50, 50),
                )
                .expect("Failed to load the Statistics down key")
        } else {
            self.render
                .canvas
                .copy(&keys, Rect::new(350, 0, 50, 50), Rect::new(140, 41, 50, 50))
                .expect("Failed to load the Statistics up key")
        }
        if self.ui_state.debug_panel {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(100, 50, 50, 50),
                    Rect::new(230, 41, 50, 50),
                )
                .expect("Failed to load the Debug down key")
        } else {
            self.render
                .canvas
                .copy(&keys, Rect::new(100, 0, 50, 50), Rect::new(230, 41, 50, 50))
                .expect("Failed to load the Debug up key")
        }
    }

    pub fn keybinds_panel(&mut self, textures: &Textures, y: i32) {
        // Loading the textures from the Textures struct
        let keys = &textures.keys;
        let keybinds_overlay = textures
            .overlay
            .get("Keybinds")
            .expect("Failed to load the Keybinds overlay");
        let random_keybinds_overlay = textures
            .overlay
            .get("Random")
            .expect("Failed to load the Random Keybinds overlay");
        let manual_keybinds_overlay = textures
            .overlay
            .get("Manual")
            .expect("Failed to load the Manual Keybinds overlay");

        self.render
            .canvas
            .copy(&keybinds_overlay, None, Rect::new(1000, y, 400, 500))
            .expect("Error generating the keybinds overlay");

        // Rendering the overlays
        self.render
            .canvas
            .copy(
                &random_keybinds_overlay,
                None,
                Rect::new(1025, 75, 350, 150),
            )
            .expect("Error");
        self.render
            .canvas
            .copy(
                &manual_keybinds_overlay,
                None,
                Rect::new(1025, 225, 350, 250),
            )
            .expect("Error");

        // Rendering the random mode keybinds
        if self.controller.mode == Mode::Random {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(0, 50, 50, 50),
                    Rect::new(1070, 145, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&keys, Rect::new(0, 0, 50, 50), Rect::new(1070, 145, 50, 50))
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::Random, ArrowKey::Up)
        {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 100, 50, 50),
                    Rect::new(1210, 95, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 0, 50, 50),
                    Rect::new(1210, 95, 50, 50),
                )
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::Random, ArrowKey::Left)
        {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(150, 150, 50, 50),
                    Rect::new(1155, 145, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(150, 50, 50, 50),
                    Rect::new(1155, 145, 50, 50),
                )
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::Random, ArrowKey::Down)
        {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 150, 50, 50),
                    Rect::new(1210, 145, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 50, 50, 50),
                    Rect::new(1210, 145, 50, 50),
                )
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::Random, ArrowKey::Right)
        {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(250, 150, 50, 50),
                    Rect::new(1265, 145, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(250, 50, 50, 50),
                    Rect::new(1265, 145, 50, 50),
                )
                .expect("Error");
        }

        // Rendering the manual keybinds
        if self.controller.mode == Mode::Manual {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(50, 50, 50, 50),
                    Rect::new(1070, 305, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(50, 0, 50, 50),
                    Rect::new(1070, 305, 50, 50),
                )
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Up {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 100, 50, 50),
                    Rect::new(1210, 255, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 0, 50, 50),
                    Rect::new(1210, 255, 50, 50),
                )
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Down {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 150, 50, 50),
                    Rect::new(1210, 305, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(200, 50, 50, 50),
                    Rect::new(1210, 305, 50, 50),
                )
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Left {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(150, 150, 50, 50),
                    Rect::new(1155, 305, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(150, 50, 50, 50),
                    Rect::new(1155, 305, 50, 50),
                )
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Right {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(250, 150, 50, 50),
                    Rect::new(1265, 305, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(250, 50, 50, 50),
                    Rect::new(1265, 305, 50, 50),
                )
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::West) {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(0, 150, 50, 50),
                    Rect::new(1100, 400, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(0, 100, 50, 50),
                    Rect::new(1100, 400, 50, 50),
                )
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::Forward) {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(50, 150, 50, 50),
                    Rect::new(1175, 400, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(50, 100, 50, 50),
                    Rect::new(1175, 400, 50, 50),
                )
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::East) {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(100, 150, 50, 50),
                    Rect::new(1250, 400, 50, 50),
                )
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(
                    &keys,
                    Rect::new(100, 100, 50, 50),
                    Rect::new(1250, 400, 50, 50),
                )
                .expect("Error");
        }
    }
}
