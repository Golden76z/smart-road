pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct Controller {
    pub random: bool,
    pub manual: bool,
    pub arrow_key: ArrowKey,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            random: true,
            manual: false,
            arrow_key: ArrowKey::None,
        }
    }

    // Switching between manual and random mode
    pub fn switch_mode(&mut self) {
        self.random = !self.random;
        self.manual = !self.manual;
    }

    // Activating random mode
    pub fn random_mode(&mut self) {
        self.random = true;
        self.manual = false;
    }

    // Activating manual mode
    pub fn manual_mode(&mut self) {
        self.manual = true;
        self.random = false;
    }

    // Activating a lane (only for manual mode)
    pub fn set_arrow_key(&mut self, arrow_key: ArrowKey) {
        if self.manual {
            self.arrow_key = arrow_key;
        }
    }
}
