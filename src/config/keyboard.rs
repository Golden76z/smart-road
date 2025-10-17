#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Mode {
    Random,
    Manual,
}

pub struct Controller {
    pub mode: Mode,
    pub arrow_key: ArrowKey,
    pub pause: bool,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            mode: Mode::Random,
            arrow_key: ArrowKey::None,
            pause: false,
        }
    }

    // Activating random mode
    pub fn random_mode(&mut self) {
        self.mode = Mode::Random;
        self.arrow_key = ArrowKey::None;
    }

    // Activating manual mode
    pub fn manual_mode(&mut self) {
        self.mode = Mode::Manual
    }

    // Activating a lane (only for manual mode)
    pub fn set_arrow_key(&mut self, arrow_key: ArrowKey) {
        if self.mode == Mode::Manual {
            self.arrow_key = arrow_key;
        }
    }

    // Pause switch
    pub fn pause_switch(&mut self) {
        self.pause = !self.pause;
    }
}
