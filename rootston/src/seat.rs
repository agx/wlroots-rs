use wlroots::{OutputHandle, XCursorTheme};
use wlroots::seat::Seat;

pub struct RootsSeat {
    seat: Seat
}


impl RootsSeat {
    pub fn configure_cursor(&mut self, output: &mut OutputHandle) {
        // TODO reset mappings
        // TODO configure device to output mappings
    }

    pub fn configure_xcursor(&mut self, output: &mut OutputHandle) {
        // TODO load from config
        let xcursor_theme = XCursorTheme::load_theme(None, 16).expect("Could not load theme");
        let xcursor = xcursor_theme.get_cursor("lefT_ptr".into())
            .expect("Could not load cursor from theme");
    }
}
