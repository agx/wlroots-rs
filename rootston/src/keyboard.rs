use wlroots::KeyboardHandler;

pub struct RootsKeyboard {}

impl RootsKeyboard {
    pub fn new() -> Self {
        RootsKeyboard {}
    }
}

impl KeyboardHandler for RootsKeyboard {}
