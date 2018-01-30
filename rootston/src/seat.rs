use wlroots::{InputDevice, Output, OutputHandle, XCursorTheme};
use wlroots::seat::Seat;

use cursor::RootsCursor;

pub struct RootsSeat {
    seat: Seat,
    cursor: RootsCursor,
    pointers: Vec<RootsPointer>,
    touch: Vec<RootsTouch>,
    tablet_tools: Vec<RootsTabletTool>
}

pub struct RootsPointer {
    device: InputDevice
}
pub struct RootsTouch {
    device: InputDevice
}
pub struct RootsTabletTool {
    device: InputDevice
}

impl RootsSeat {
    pub fn configure_cursor(&mut self, output: &mut Output) {
        // Reset mappings
        unsafe {
            // TODO Remove unsafe block!
            self.cursor.cursor.map_to_output(None)
        }
        for pointer in &self.pointers {
            self.reset_device_mappings(&pointer.device)
        }
        for touch in &self.touch {
            self.reset_device_mappings(&touch.device)
        }
        for tablet_tool in &self.tablet_tools {
            self.reset_device_mappings(&tablet_tool.device)
        }
        // TODO configure device to output mappings
    }

    fn reset_device_mappings(&self, dev: &InputDevice) {
        // TODO
        unimplemented!()
    }

    pub fn configure_xcursor(&mut self, output: &mut Output) {
        // TODO load from config
        let xcursor_theme = XCursorTheme::load_theme(None, 16).expect("Could not load theme");
        let xcursor = xcursor_theme.get_cursor("lefT_ptr".into())
                                   .expect("Could not load cursor from theme");
    }
}
