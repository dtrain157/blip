use super::event::{Event, EventCategory, EventType};
use std::fmt;

/***********************************************************/

pub struct KeyPressedEvent {
    pub handled: bool,
    pub keycode: u32,
    pub repeat_count: u32,
}

impl Event for KeyPressedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::KEYBOARD | EventCategory::INPUT
    }
}

impl fmt::Display for KeyPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Pressed Event: {} ({} repeats)", self.keycode, self.repeat_count)
    }
}

/***********************************************************/

pub struct KeyReleasedEvent {
    pub handled: bool,
    pub keycode: u32,
}

impl Event for KeyReleasedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::KeyReleased
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::KEYBOARD | EventCategory::INPUT
    }
}

impl fmt::Display for KeyReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Released Event: {}", self.keycode)
    }
}

/***********************************************************/

pub struct KeyTypedEvent {
    pub handled: bool,
    pub keycode: u32,
}

impl Event for KeyTypedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::KeyTyped
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::KEYBOARD | EventCategory::INPUT
    }
}

impl fmt::Display for KeyTypedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Typed Event: {}", self.keycode)
    }
}

/***********************************************************/
