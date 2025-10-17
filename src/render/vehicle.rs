use sdl2::{
    image::LoadTexture, pixels::Color, rect::Rect, render::TextureCreator, video::WindowContext,
};

use crate::config::{GameSettings, HitboxType, Lane, VEHICLE_HEIGHT, VEHICLE_WIDTH};

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

        for ((lane, dir), vehicle_lane) in &self.lanes.lanes {
            let queue = vehicle_lane.lock().unwrap();

            for vehicle in queue.iter() {
                let (x, y) = vehicle.coordinates;
                let texture = &car_textures[vehicle.color as usize % car_textures.len()];

                // Draw car at its coordinates
                let dest = Rect::new(x as i32, y as i32, VEHICLE_WIDTH, VEHICLE_HEIGHT);

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
                match vehicle.hitbox_type {
                    HitboxType::Big => self.render.canvas.set_draw_color(Color::RGB(0, 255, 0)),
                    HitboxType::Medium => {
                        self.render.canvas.set_draw_color(Color::RGB(255, 255, 0))
                    }
                    HitboxType::Small => self.render.canvas.set_draw_color(Color::RGB(255, 128, 0)),
                    HitboxType::Stop => self.render.canvas.set_draw_color(Color::RGB(255, 0, 0)),
                }
                self.render.canvas.draw_rect(vehicle.hitbox.unwrap());
            }
        }
    }
}
