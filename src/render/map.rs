use sdl2::{image::LoadTexture, rect::Rect};

use crate::render::renderer::Renderer;

use sdl2::rect::Point;
use sdl2::render::BlendMode;
use sdl2::render::Texture;

// Example: draw rotated texture

impl Renderer {
    fn draw_rotated(
        &mut self,
        texture: &Texture,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        angle: f64,
    ) -> Result<(), String> {
        // Where on screen you want it
        let dst = Rect::new(x, y, w, h);

        // Center of rotation: middle of the rect (can be anywhere!)
        let center = Point::new((w / 2) as i32, (h / 2) as i32);

        // Copy with rotation
        self.canvas.copy_ex(
            texture,
            None,      // source rect (None = whole texture)
            Some(dst), // destination rect
            angle,     // angle in degrees
            center,    // rotation center
            false,     // flip horizontal
            false,     // flip vertical
        )
    }

    pub fn create_map(&mut self) {
        // Load a PNG into a surface, then into a texture
        let texture_creator = self.canvas.texture_creator();

        let lane_vertical = texture_creator
            .load_texture("../../assets/lane_vertical.png")
            .unwrap();
        let lane_horizontal = texture_creator
            .load_texture("../../assets/lane_horizontal.png")
            .unwrap();
        let corner = texture_creator
            .load_texture("../../assets/bottom-left.png")
            .unwrap();
        let center = texture_creator
            .load_texture("../../assets/center.png")
            .unwrap();

        // Drawing the 4 corners of the map
        {
            let _ = self.draw_rotated(&corner, 0, 0, 350, 350, 90.0);
            let _ = self.draw_rotated(&corner, 650, 0, 350, 350, 180.0);
            let _ = self.draw_rotated(&corner, 0, 650, 350, 350, 0.0);
            let _ = self.draw_rotated(&corner, 650, 650, 350, 350, 270.0);
        }

        // Drawing the 4 lanes of the map
        {
            let _ = self.draw_rotated(&lane_vertical, 350, 0, 300, 350, 180.0);
            let _ = self.draw_rotated(&lane_vertical, 350, 650, 300, 350, 0.0);

            let _ = self.draw_rotated(&lane_horizontal, 0, 350, 350, 300, 0.0);
            let _ = self.draw_rotated(&lane_horizontal, 650, 350, 350, 300, 180.0);
        }

        // Drawing the center of the map
        {
            let _ = self.draw_rotated(&center, 350, 350, 300, 300, 0.0);
        }
    }
}
