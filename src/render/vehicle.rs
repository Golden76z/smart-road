use sdl2::{pixels::Color, rect::Rect};

use crate::{
    config::{GameSettings, HitboxType, VEHICLE_HEIGHT, VEHICLE_WIDTH},
    render::textures::Textures,
};

impl<'a> GameSettings<'a> {
    pub fn render_vehicles(&mut self, textures: &Textures) {
        for ((_, _), vehicle_lane) in &self.lanes.lanes {
            let queue = vehicle_lane.lock().unwrap();

            for vehicle in queue.iter() {
                let (x, y) = vehicle.coordinates;
                let texture = &textures.vehicle;

                // Matching the sprite depending on the sprite angle
                let dest = Rect::new(x as i32, y as i32, VEHICLE_WIDTH, VEHICLE_HEIGHT);
                let sprite_position: i32 = (vehicle.color * 50) as i32;
                match vehicle.sprite_angle {
                    0.0 => self.render.canvas.copy(
                        &texture,
                        Rect::new(150, sprite_position, 50, 50),
                        dest,
                    ),
                    90.0 => self.render.canvas.copy(
                        &texture,
                        Rect::new(0, sprite_position, 50, 50),
                        dest,
                    ),
                    180.0 => self.render.canvas.copy(
                        &texture,
                        Rect::new(50, sprite_position, 50, 50),
                        dest,
                    ),
                    270.0 => self.render.canvas.copy(
                        &texture,
                        Rect::new(100, sprite_position, 50, 50),
                        dest,
                    ),
                    _ => unreachable!(),
                }
                .expect("Error trying to read vehicle sprite angle");

                if self.ui_state.debug_panel {
                    // Changing the drawing color depending on the hitbox size
                    match vehicle.hitbox_type {
                        HitboxType::Big => self.render.canvas.set_draw_color(Color::RGB(0, 255, 0)),
                        HitboxType::Medium => {
                            self.render.canvas.set_draw_color(Color::RGB(255, 255, 0))
                        }
                        HitboxType::Small => {
                            self.render.canvas.set_draw_color(Color::RGB(255, 180, 0))
                        }
                        HitboxType::VerySmall => {
                            self.render.canvas.set_draw_color(Color::RGB(255, 120, 0))
                        }
                        HitboxType::AlmostStop => {
                            self.render.canvas.set_draw_color(Color::RGB(255, 60, 0));
                        }
                        HitboxType::Stop => {
                            self.render.canvas.set_draw_color(Color::RGB(255, 0, 0))
                        }
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
}
