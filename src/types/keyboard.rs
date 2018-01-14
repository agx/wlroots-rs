//! TODO Documentation
use std::fmt;
use std::rc::{Rc, Weak};

use wlroots_sys::{wlr_input_device, wlr_keyboard, wlr_keyboard_get_modifiers, wlr_keyboard_led,
                  wlr_keyboard_led_update, wlr_keyboard_modifier, wlr_keyboard_set_keymap,
                  xkb_keymap};

#[derive(Debug)]
pub struct Keyboard {
    /// The structure that ensures weak handles to this structure are still alive.
    ///
    /// They contain weak handles, and will safely not use dead memory when this
    /// is freed by wlroots.
    ///
    /// If this is `None`, then this is from an upgraded `KeyboardHandle`, and
    /// the operations are **unchecked**.
    /// This is means safe operations might fail, but only if you use the unsafe
    /// marked function `upgrade` on a `KeyboardHandle`.
    liveliness: Option<Rc<()>>,
    /// The device that refers to this keyboard.
    device: *mut wlr_input_device,
    /// The underlying keyboard data.
    keyboard: *mut wlr_keyboard
}

#[derive(Debug)]
pub struct KeyboardHandle {
    /// The Rc that ensures that this handle is still alive.
    ///
    /// When wlroots deallocates the keyboard associated with this handle,
    handle: Weak<()>,
    /// The device that refers to this keyboard.
    device: *mut wlr_input_device,
    /// The underlying keyboard data.
    keyboard: *mut wlr_keyboard
}

impl Keyboard {
    pub(crate) unsafe fn from_input_device(device: *mut wlr_input_device) -> Option<Self> {
        use wlroots_sys::wlr_input_device_type::*;
        match (*device).type_ {
            WLR_INPUT_DEVICE_KEYBOARD => {
                let keyboard = (*device).__bindgen_anon_1.keyboard;
                Some(Keyboard { liveliness: Some(Rc::new(())),
                                device,
                                keyboard })
            }
            _ => None
        }
    }

    unsafe fn from_handle(handle: &KeyboardHandle) -> Self {
        Keyboard { liveliness: None,
                   device: handle.input_device(),
                   keyboard: handle.keyboard_ptr() }
    }

    /// Gets the wlr_keyboard associated with this KeyboardHandle.
    pub unsafe fn keyboard_ptr(&self) -> *mut wlr_keyboard {
        self.keyboard
    }

    /// Gets the wlr_input_device associated with this KeyboardHandle
    pub unsafe fn input_device(&self) -> *mut wlr_input_device {
        self.device
    }

    // TODO: Implement keymap wrapper?
    pub fn set_keymap(&mut self, keymap: *mut xkb_keymap) {
        unsafe {
            wlr_keyboard_set_keymap(self.keyboard, keymap);
        }
    }

    pub fn update_led(&mut self, leds: KeyboardLed) {
        unsafe {
            wlr_keyboard_led_update(self.keyboard, leds.bits() as u32);
        }
    }

    pub fn get_modifiers(&self) -> KeyboardModifier {
        unsafe {
            KeyboardModifier::from_bits_truncate(wlr_keyboard_get_modifiers(self.keyboard))
        }
    }

    /// Creates a weak reference to a `Keyboard`.
    ///
    /// # Panics
    /// If this `Keyboard` is a previously upgraded `KeyboardHandle`,
    /// then this function will panic.
    pub fn weak_reference(&self) -> KeyboardHandle {
        let arc = self.liveliness.as_ref()
                      .expect("Cannot downgrade previously upgraded KeyboardHandle!");
        KeyboardHandle { handle: Rc::downgrade(arc),
                         device: self.device,
                         keyboard: self.keyboard }
    }
}

impl KeyboardHandle {
    /// Upgrades the keyboard handle to a reference to the backing `Keyboard`.
    ///
    /// # Unsafety
    /// This function is unsafe, because it creates a lifetime bound to
    /// KeyboardHandle, which may live forever..
    /// But no output lives forever and might be disconnected at any time.
    pub unsafe fn upgrade(&self) -> Option<Keyboard> {
        self.handle.upgrade()
        // NOTE
        // We drop the upgrade here because we don't want to cause a memory leak!
            .map(|_| Keyboard::from_handle(self))
    }

    /// Gets the wlr_input_device associated with this KeyboardHandle
    pub unsafe fn input_device(&self) -> *mut wlr_input_device {
        self.device
    }

    /// Gets the wlr_keyboard associated with this KeyboardHandle.
    pub unsafe fn keyboard_ptr(&self) -> *mut wlr_keyboard {
        self.keyboard
    }
}

bitflags! {
    pub struct KeyboardLed: u32 {
        const WLR_LED_NUM_LOCK = wlr_keyboard_led::WLR_LED_NUM_LOCK as u32;
        const WLR_LED_CAPS_LOCK = wlr_keyboard_led::WLR_LED_CAPS_LOCK as u32;
        const WLR_LED_SCROLL_LOCK = wlr_keyboard_led::WLR_LED_SCROLL_LOCK as u32;
    }
}

bitflags! {
    pub struct KeyboardModifier: u32 {
        const WLR_MODIFIER_SHIFT = wlr_keyboard_modifier::WLR_MODIFIER_SHIFT as u32;
        const WLR_MODIFIER_CAPS = wlr_keyboard_modifier::WLR_MODIFIER_CAPS as u32;
        const WLR_MODIFIER_CTRL = wlr_keyboard_modifier::WLR_MODIFIER_CTRL as u32;
        const WLR_MODIFIER_ALT = wlr_keyboard_modifier::WLR_MODIFIER_ALT as u32;
        const WLR_MODIFIER_MOD2 = wlr_keyboard_modifier::WLR_MODIFIER_MOD2 as u32;
        const WLR_MODIFIER_MOD3 = wlr_keyboard_modifier::WLR_MODIFIER_MOD3 as u32;
        const WLR_MODIFIER_LOGO = wlr_keyboard_modifier::WLR_MODIFIER_LOGO as u32;
        const WLR_MODIFIER_MOD5 = wlr_keyboard_modifier::WLR_MODIFIER_MOD5 as u32;
    }
}

impl fmt::Display for KeyboardModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mod_vec = vec![("Shift", KeyboardModifier::WLR_MODIFIER_SHIFT),
                           ("Caps lock", KeyboardModifier::WLR_MODIFIER_CAPS),
                           ("Ctrl", KeyboardModifier::WLR_MODIFIER_CTRL),
                           ("Alt", KeyboardModifier::WLR_MODIFIER_ALT),
                           ("Mod2", KeyboardModifier::WLR_MODIFIER_MOD2),
                           ("Mod3", KeyboardModifier::WLR_MODIFIER_MOD3),
                           ("Logo", KeyboardModifier::WLR_MODIFIER_LOGO),
                           ("Mod5", KeyboardModifier::WLR_MODIFIER_MOD5)];

        let mods: Vec<&str> = mod_vec.into_iter()
                                     .filter(|&(_, flag)| self.contains(flag))
                                     .map(|(st, _)| st)
                                     .collect();

        write!(formatter, "{:?}", mods)
    }
}
