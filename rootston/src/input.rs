//! Input manager for rootston. Handles all of the seat connections.

use wlroots::InputManagerHandler;

pub struct RootsInput {}

impl RootsInput {
    pub fn new() -> RootsInput {
        RootsInput {}
    }
}

impl InputManagerHandler for RootsInput {}
