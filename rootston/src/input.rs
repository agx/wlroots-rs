//! Input manager for rootston. Handles all of the seat connections.

use wlroots::{InputManagerHandler, KeyboardHandler, Compositor};
use wlroots::types::KeyboardHandle;

use keyboard::RootsKeyboard;
use server::RootsServer;

pub struct RootsInput {}

impl RootsInput {
    pub fn new() -> RootsInput {
        RootsInput {}
    }
}

impl InputManagerHandler for RootsInput {
    fn keyboard_added(&mut self,
                     compositor: &mut Compositor,
                     _: &mut KeyboardHandle) -> Option<Box<KeyboardHandler>> {
        let state: &mut RootsServer = compositor.into();
        state.seats.push(RootsInput::Keyboard)
        Some(Box::new(RootsKeyboard::new()))
    }
}
