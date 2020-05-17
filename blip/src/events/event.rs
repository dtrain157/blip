use bitflags::bitflags;
use std::fmt;

pub enum EventType {
    None,

    //Window events
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,

    // App events
    AppTick,
    AppUpdate,
    AppRender,

    //Keyboard events
    KeyPressed,
    KeyReleased,
    KeyTyped,

    //Mouse events
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

bitflags! {
    pub struct EventCategory: u8 {
        const NONE          = 0b00000000;
        const APPLICATION   = 0b00000001;
        const INPUT         = 0b00000010;
        const KEYBOARD      = 0b00000100;
        const MOUSE         = 0b00001000;
        const MOUSEBUTTON   = 0b00010000;
    }
}

pub trait Event: fmt::Display {
    fn get_event_type(&self) -> EventType;
    fn get_category_flags(&self) -> EventCategory;
    #[inline]
    fn is_in_category(&self, ec: EventCategory) -> bool {
        (self.get_category_flags() & ec) != EventCategory::NONE
    }
}
