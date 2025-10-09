use crate::config::{ArrowKey, Direction, GameSettings, Mode};
use sdl2::{image::LoadTexture, rect::Rect, render::TextureCreator, video::WindowContext};

impl<'a> GameSettings<'a> {
    pub fn keybinds_panel(&mut self, texture_creator: &TextureCreator<WindowContext>, y: i32) {
        let keybinds_overlay = texture_creator
            .load_texture("../../assets/images/overlay/setting_overlay.png")
            .unwrap();

        let random_keybinds_overlay = texture_creator
            .load_texture("../../assets/images/overlay/random_keybinds_overlay.png")
            .unwrap();
        let manual_keybinds_overlay = texture_creator
            .load_texture("../../assets/images/overlay/manual_keybinds_overlay.png")
            .unwrap();

        let random_key = texture_creator
            .load_texture("../../assets/images/keys/random_up.png")
            .unwrap();
        let random_key_down = texture_creator
            .load_texture("../../assets/images/keys/random_down.png")
            .unwrap();

        let manual_key = texture_creator
            .load_texture("../../assets/images/keys/manual_up.png")
            .unwrap();
        let manual_key_down = texture_creator
            .load_texture("../../assets/images/keys/manual_down.png")
            .unwrap();

        let up_up = texture_creator
            .load_texture("../../assets/images/keys/up_up.png")
            .unwrap();
        let up_down = texture_creator
            .load_texture("../../assets/images/keys/up_down.png")
            .unwrap();

        let down_up = texture_creator
            .load_texture("../../assets/images/keys/down_up.png")
            .unwrap();
        let down_down = texture_creator
            .load_texture("../../assets/images/keys/down_down.png")
            .unwrap();

        let left_up = texture_creator
            .load_texture("../../assets/images/keys/left_up.png")
            .unwrap();
        let left_down = texture_creator
            .load_texture("../../assets/images/keys/left_down.png")
            .unwrap();

        let right_up = texture_creator
            .load_texture("../../assets/images/keys/right_up.png")
            .unwrap();
        let right_down = texture_creator
            .load_texture("../../assets/images/keys/right_down.png")
            .unwrap();

        let first_up = texture_creator
            .load_texture("../../assets/images/keys/1_up.png")
            .unwrap();
        let first_down = texture_creator
            .load_texture("../../assets/images/keys/1_down.png")
            .unwrap();

        let second_up = texture_creator
            .load_texture("../../assets/images/keys/2_up.png")
            .unwrap();
        let second_down = texture_creator
            .load_texture("../../assets/images/keys/2_down.png")
            .unwrap();

        let third_up = texture_creator
            .load_texture("../../assets/images/keys/3_up.png")
            .unwrap();
        let third_down = texture_creator
            .load_texture("../../assets/images/keys/3_down.png")
            .unwrap();

        self.render
            .canvas
            .copy(&keybinds_overlay, None, Rect::new(1000, y, 400, 500))
            .expect("Error generating the keybinds overlay");

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
        if self.controller.random {
            self.render
                .canvas
                .copy(&random_key_down, None, Rect::new(1070, 145, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&random_key, None, Rect::new(1070, 145, 50, 50))
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::random, ArrowKey::Up)
        {
            self.render
                .canvas
                .copy(&up_down, None, Rect::new(1210, 95, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&up_up, None, Rect::new(1210, 95, 50, 50))
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::random, ArrowKey::Left)
        {
            self.render
                .canvas
                .copy(&left_down, None, Rect::new(1155, 145, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&left_up, None, Rect::new(1155, 145, 50, 50))
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::random, ArrowKey::Down)
        {
            self.render
                .canvas
                .copy(&down_down, None, Rect::new(1210, 145, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&down_up, None, Rect::new(1210, 145, 50, 50))
                .expect("Error");
        }

        if self
            .spawn_manager
            .is_key_pressed_random(Mode::random, ArrowKey::Right)
        {
            self.render
                .canvas
                .copy(&right_down, None, Rect::new(1265, 145, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&right_up, None, Rect::new(1265, 145, 50, 50))
                .expect("Error");
        }

        // Rendering the manual keybinds
        if self.controller.manual {
            self.render
                .canvas
                .copy(&manual_key_down, None, Rect::new(1070, 305, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&manual_key, None, Rect::new(1070, 305, 50, 50))
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Up {
            self.render
                .canvas
                .copy(&up_down, None, Rect::new(1210, 255, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&up_up, None, Rect::new(1210, 255, 50, 50))
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Down {
            self.render
                .canvas
                .copy(&down_down, None, Rect::new(1210, 305, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&down_up, None, Rect::new(1210, 305, 50, 50))
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Left {
            self.render
                .canvas
                .copy(&left_down, None, Rect::new(1155, 305, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&left_up, None, Rect::new(1155, 305, 50, 50))
                .expect("Error");
        }

        if self.controller.arrow_key == ArrowKey::Right {
            self.render
                .canvas
                .copy(&right_down, None, Rect::new(1265, 305, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&right_up, None, Rect::new(1265, 305, 50, 50))
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::West) {
            self.render
                .canvas
                .copy(&first_down, None, Rect::new(1100, 390, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&first_up, None, Rect::new(1100, 390, 50, 50))
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::Forward) {
            self.render
                .canvas
                .copy(&second_down, None, Rect::new(1175, 390, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&second_up, None, Rect::new(1175, 390, 50, 50))
                .expect("Error");
        }

        if self.spawn_manager.is_key_pressed_manual(Direction::East) {
            self.render
                .canvas
                .copy(&third_down, None, Rect::new(1250, 390, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&third_up, None, Rect::new(1250, 390, 50, 50))
                .expect("Error");
        }
    }
}
