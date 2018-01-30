//! Input manager for rootston. Handles all of the seat connections.

use wlroots::{InputManagerHandler, KeyboardHandler, Compositor};
use wlroots::types::Keyboard;

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
                     _: &mut Keyboard) -> Option<Box<KeyboardHandler>> {
        let state: &mut RootsServer = compositor.into();
        //state.seats.push(RootsInput::);
        Some(Box::new(RootsKeyboard::new()))
    }
}
