//! Output manager for rootson. Handles connecting and disconnecting outputs.

use wlroots::OutputHandler;

pub struct RootsOutput {}

impl RootsOutput {
    pub fn new() -> RootsOutput {
        RootsOutput {}
    }
}

impl OutputHandler for RootsOutput {}
