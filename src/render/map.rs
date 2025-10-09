use sdl2::{image::LoadTexture, rect::Point, rect::Rect, render::Texture};

use crate::render::renderer::Renderer;

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
        // Rect with the positions and dimensions
        let dst = Rect::new(x, y, w, h);

        // Center of rotation: middle of the rect
        let center = Point::new((w / 2) as i32, (h / 2) as i32);

        // Copy with rotation
        self.canvas.copy_ex(
            texture,
            // source rect (None = whole texture)
            None,
            Some(dst),
            angle,
            center,
            false,
            false,
        )
    }

    pub fn create_map(&mut self) {
        // Load a PNG into a surface, then into a texture
        let texture_creator = self.canvas.texture_creator();

        // Loading the textures
        let lane_vertical = texture_creator
            .load_texture("../../assets/images/road/lane_vertical.png")
            .unwrap();
        let lane_horizontal = texture_creator
            .load_texture("../../assets/images/road/lane_horizontal.png")
            .unwrap();
        let corner = texture_creator
            .load_texture("../../assets/images/road/bottom-left.png")
            .unwrap();
        let center = texture_creator
            .load_texture("../../assets/images/road/center.png")
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
            // Drawing top and bottom lanes
            self.draw_rotated(&lane_vertical, 350, 0, 300, 350, 180.0)
                .expect("[map.rs]: Error drawing top lane");
            self.draw_rotated(&lane_vertical, 350, 650, 300, 350, 0.0)
                .expect("[map.rs]: Error drawing bottom lane");

            // Drawing left and right lanes
            self.draw_rotated(&lane_horizontal, 0, 350, 350, 300, 0.0)
                .expect("[map.rs]: Error drawing left lane");
            self.draw_rotated(&lane_horizontal, 650, 350, 350, 300, 180.0)
                .expect("[map.rs]: Error drawing right lane");
        }

        {
            // Drawing the center of the map
            let _ = self.draw_rotated(&center, 350, 350, 300, 300, 0.0);
        }
    }
}
