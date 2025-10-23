use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::config::{MAX_DEBUG_MESSAGES, UiState};

#[derive(Clone, Copy)]
pub enum MessageType {
    Setting,
    Info,
    Warning,
    Error,
}

pub struct Broadcaster<'a> {
    font: Font<'a, 'a>,
    messages: Vec<(String, MessageType)>,
    // Live status lines updated each frame (not persisted in messages)
    status_lines: Vec<String>,
}

impl<'a> Broadcaster<'a> {
    pub fn new(ttf_context: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font("../../assets/font/Pixellari.ttf", 16)
            .expect("Failed to load font");

        Self {
            font,
            messages: Vec::new(),
            status_lines: Vec::new(),
        }
    }

    /// Log a message with a specific type (Info, Warning, Error)
    pub fn log(&mut self, msg: &str, msg_type: MessageType) {
        let prefix = match msg_type {
            MessageType::Setting => "[SETTING]",
            MessageType::Info => "[INFO]",
            MessageType::Warning => "[WARN]",
            MessageType::Error => "[ERROR]",
        };

        println!("{} {}", prefix, msg);
        self.messages.push((msg.to_string(), msg_type));

        if self.messages.len() > MAX_DEBUG_MESSAGES as usize {
            self.messages.remove(0);
        }
    }

    /// Internal helper to pick colors by message type
    fn colors_for(msg_type: MessageType) -> (Color, Color, Color) {
        match msg_type {
            MessageType::Setting => (
                Color::RGB(150, 150, 150),
                Color::RGB(30, 30, 30),
                Color::RGB(80, 80, 80),
            ),
            MessageType::Info => (
                Color::RGB(150, 150, 220),
                Color::RGB(40, 40, 60),
                Color::RGB(80, 80, 120),
            ),
            MessageType::Warning => (
                Color::RGB(180, 180, 120),
                Color::RGB(80, 50, 15),
                Color::RGB(90, 70, 0),
            ),
            MessageType::Error => (
                Color::RGB(220, 120, 120),
                Color::RGB(90, 20, 20),
                Color::RGB(120, 0, 0),
            ),
        }
    }

    /// Render messages on-screen with boxes and colors
    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        ui_state: &UiState,
    ) {
        // Only draw when debug panels are off
        if !ui_state.keybinds_panel && !ui_state.statistic_panel {
            let mut y = 90;

            // First draw any live status lines (these are refreshed each frame)
            for line in &self.status_lines {
                let (text_color, bg_color, border_color) = Self::colors_for(MessageType::Info);
                if let Ok(surface) = self.font.render(line).solid(text_color) {
                    if let Ok(texture) = surface.as_texture(texture_creator) {
                        let query = texture.query();
                        let bg_rect = Rect::new(1028, y - 2, query.width + 16, query.height + 12);
                        canvas.set_draw_color(bg_color);
                        let _ = canvas.fill_rect(bg_rect);
                        canvas.set_draw_color(border_color);
                        let _ = canvas.draw_rect(bg_rect);
                        let dst = Rect::new(1035, y + 4, query.width, query.height);
                        let _ = canvas.copy(&texture, None, Some(dst));
                        y += (query.height + 16) as i32;
                    }
                }
            }

            for (msg, msg_type) in &self.messages {
                let (text_color, bg_color, border_color) = Self::colors_for(*msg_type);

                if let Ok(surface) = self.font.render(msg).solid(text_color) {
                    if let Ok(texture) = surface.as_texture(texture_creator) {
                        let query = texture.query();

                        // Draw background box + border
                        let bg_rect = Rect::new(1028, y - 2, query.width + 16, query.height + 12);

                        // background
                        canvas.set_draw_color(bg_color);
                        let _ = canvas.fill_rect(bg_rect);

                        // border
                        canvas.set_draw_color(border_color);
                        let _ = canvas.draw_rect(bg_rect);

                        // draw text on top
                        let dst = Rect::new(1035, y + 4, query.width, query.height);
                        let _ = canvas.copy(&texture, None, Some(dst));

                        y += (query.height + 16) as i32;
                    }
                }


            }
        }
    }

    /// Set live status lines to be rendered in the debug panel. Overwrites previous lines.
    pub fn set_status_lines(&mut self, lines: Vec<String>) {
        self.status_lines = lines;
    }

    pub fn text_texture<'b>(
        &self,
        texture_creator: &'b TextureCreator<WindowContext>,
        text: &str,
        color: Color,
    ) -> Option<sdl2::render::Texture<'b>> {
        if let Ok(surface) = self.font.render(text).blended(color) {
            if let Ok(texture) = surface.as_texture(texture_creator) {
                return Some(texture);
            }
        }
        None
    }
}
