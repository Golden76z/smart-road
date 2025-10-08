use crate::config::GameSettings;
use sdl2::{image::LoadTexture, rect::Rect, render::TextureCreator, video::WindowContext};

impl<'a> GameSettings<'a> {
    pub fn keybinds_panel(&mut self, texture_creator: &TextureCreator<WindowContext>, y: i32) {
        let keybinds_overlay = texture_creator
            .load_texture("../../assets/images/overlay/setting_overlay.png")
            .unwrap();

        let random_key = texture_creator
            .load_texture("../../assets/images/keys/random_up.png")
            .unwrap();
        let random_key_down = texture_creator
            .load_texture("../../assets/images/keys/random_down.png")
            .unwrap();
        let up_up = texture_creator
            .load_texture("../../assets/images/keys/up_up.png")
            .unwrap();
        let down_up = texture_creator
            .load_texture("../../assets/images/keys/down_up.png")
            .unwrap();
        let left_up = texture_creator
            .load_texture("../../assets/images/keys/left_up.png")
            .unwrap();
        let right_up = texture_creator
            .load_texture("../../assets/images/keys/right_up.png")
            .unwrap();

        self.render
            .canvas
            .copy(&keybinds_overlay, None, Rect::new(1000, y, 400, 500))
            .expect("Error generating the keybinds overlay");

        if self.controller.random {
            self.render
                .canvas
                .copy(&random_key_down, None, Rect::new(1070, 120, 50, 50))
                .expect("Error");
        } else {
            self.render
                .canvas
                .copy(&random_key, None, Rect::new(1070, 120, 50, 50))
                .expect("Error");
        }

        self.render
            .canvas
            .copy(&up_up, None, Rect::new(1210, 70, 50, 50))
            .expect("Error");

        self.render
            .canvas
            .copy(&left_up, None, Rect::new(1155, 120, 50, 50))
            .expect("Error");

        self.render
            .canvas
            .copy(&down_up, None, Rect::new(1210, 120, 50, 50))
            .expect("Error");

        self.render
            .canvas
            .copy(&right_up, None, Rect::new(1265, 120, 50, 50))
            .expect("Error");
    }
}
