use sdl2::{image::LoadTexture, rect::Rect, render::TextureCreator, video::WindowContext};

use crate::config::{GameSettings, Lane, VEHICLE_HEIGHT, VEHICLE_WIDTH};

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

                // Determine sprite rotation based on lane and direction
                let angle = match lane {
                    Lane::Up => 180.0,
                    Lane::Bottom => 0.0,
                    Lane::Left => 90.0,
                    Lane::Right => 270.0,
                };

                // Draw car at its coordinates
                let dest = Rect::new(x as i32, y as i32, VEHICLE_WIDTH, VEHICLE_HEIGHT);

                self.render
                    .canvas
                    .copy_ex(&texture, None, dest, angle, None, false, false)
                    .unwrap();
            }
        }
    }
}
