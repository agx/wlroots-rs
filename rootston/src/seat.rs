use wlroots::{Seat, OutputHandle};

pub struct RootsSeat {
    seat: Seat
}


impl RootsSeat {
    pub fn configure_cursor(&mut self, output: &mut OutputHandle) {
        // TODO reset mappings
        // TODO configure device to output mappings
    }

    pub fn configure_xcursor(&mut self, output: &mut OutputHandle) {
        // TODO Load xcursor theme (specified in config) and bind it to all outputs
    }
}
