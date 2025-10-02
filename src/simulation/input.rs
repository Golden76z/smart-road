use crate::config::UiState;
use sdl2::event::Event;

pub fn input_listener(event: Event, ui: &mut UiState) -> Result<(), String> {
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
                ui.show_keybinds_panel = !ui.show_keybinds_panel;
                Ok(())
            }
            // Toggle the debug panel (OFF by default)
            sdl2::keyboard::Keycode::F2 => {
                ui.show_debug_panel = !ui.show_debug_panel;
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
