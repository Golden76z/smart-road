use sdl2::{
    image::LoadTexture, pixels::Color, rect::Rect, render::TextureCreator, video::WindowContext,
};

use crate::config::{GameSettings, HitboxType, VEHICLE_HEIGHT, VEHICLE_WIDTH};

impl<'a> GameSettings<'a> {
    pub fn render_vehicles(&mut self, texture_creator: &TextureCreator<WindowContext>) {
        let car_textures = vec![
            texture_creator
                .load_texture("../../assets/images/cars/car_1.png")
                .unwrap(),
            texture_creator
                .load_texture("../../assets/images/cars/car_2.png")
                .unwrap(),
            texture_creator
                .load_texture("../../assets/images/cars/car_3.png")
                .unwrap(),
            texture_creator
                .load_texture("../../assets/images/cars/car_4.png")
                .unwrap(),
        ];

        for ((_, _), vehicle_lane) in &self.lanes.lanes {
            let queue = vehicle_lane.lock().unwrap();

            for vehicle in queue.iter() {
                let (x, y) = vehicle.coordinates;
                let texture = &car_textures[vehicle.color as usize % car_textures.len()];

                // Draw car at its coordinates
                let dest = Rect::new(x as i32, y as i32, VEHICLE_WIDTH, VEHICLE_HEIGHT);

                // Rendering the vehicle texture
                self.render
                    .canvas
                    .copy_ex(
                        &texture,
                        None,
                        dest,
                        vehicle.sprite_angle,
                        None,
                        false,
                        false,
                    )
                    .unwrap();

                // Changing the drawing color depending on the hitbox size
                match vehicle.hitbox_type {
                    HitboxType::Big => self.render.canvas.set_draw_color(Color::RGB(0, 255, 0)),
                    HitboxType::Medium => {
                        self.render.canvas.set_draw_color(Color::RGB(255, 255, 0))
                    }
                    HitboxType::Small => self.render.canvas.set_draw_color(Color::RGB(255, 160, 0)),
                    HitboxType::VerySmall => {
                        self.render.canvas.set_draw_color(Color::RGB(255, 80, 0))
                    }
                    HitboxType::Stop => self.render.canvas.set_draw_color(Color::RGB(255, 0, 0)),
                }

                // Rendering the 1 or 2 parts of the vehicle hitbox
                for rect in [&vehicle.hitbox.0, &vehicle.hitbox.1].into_iter().flatten() {
                    self.render
                        .canvas
                        .draw_rect(*rect)
                        .expect("Error drawing hitbox");
                }
            }
        }
    }
}
