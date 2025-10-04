use crate::config::GameSettings;
use sdl2::event::Event;

impl GameSettings {
    pub fn input_listener(&mut self, event: Event) -> Result<(), String> {
        match event {
            // Listening for Key presses
            sdl2::event::Event::KeyDown {
                keycode: Some(k), ..
            } => match k {
                // Closing the programm when Escape key is pressed
                sdl2::keyboard::Keycode::Escape => {
                    return Err("Program stopped".to_string());
                }
                // Toggle the keybinds panel (ON by default)
                sdl2::keyboard::Keycode::F1 => {
                    self.ui_state.toggle_keybinds();
                    Ok(())
                }
                // Toggle the debug panel (OFF by default)
                sdl2::keyboard::Keycode::F2 => {
                    self.ui_state.toggle_debug();
                    Ok(())
                }
                _ => Ok(()),
            },
            // Closing the programm when closing the window
            sdl2::event::Event::Quit { .. } => {
                return Err("Program stopped".to_string());
            }
            _ => Ok(()),
        }
    }
}
