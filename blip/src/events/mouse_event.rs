use super::event::{Event, EventCategory, EventType};
use std::fmt;

/***********************************************************/

pub struct MouseMovedEvent {
    pub handled: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl Event for MouseMovedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::MouseMoved
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::MOUSE | EventCategory::INPUT
    }
}

impl fmt::Display for MouseMovedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse Moved Event: ({}, {})", self.mouse_x, self.mouse_y)
    }
}

/***********************************************************/

pub struct MouseScrolledEvent {
    pub handled: bool,
    pub mouse_x_offset: f32,
    pub mouse_y_offset: f32,
}

impl Event for MouseScrolledEvent {
    fn get_event_type(&self) -> EventType {
        EventType::MouseScrolled
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::MOUSE | EventCategory::INPUT
    }
}

impl fmt::Display for MouseScrolledEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse Scrolled Event: ({}, {})", self.mouse_x_offset, self.mouse_y_offset)
    }
}

/***********************************************************/

pub struct MouseButtonPressedEvent {
    pub handled: bool,
    pub mouse_button: u8,
}

impl Event for MouseButtonPressedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::MouseButtonPressed
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::MOUSE | EventCategory::INPUT
    }
}

impl fmt::Display for MouseButtonPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse Button Pressed Event: {}", self.mouse_button)
    }
}

/***********************************************************/

pub struct MouseButtonReleasedEvent {
    pub handled: bool,
    pub mouse_button: u8,
}

impl Event for MouseButtonReleasedEvent {
    fn get_event_type(&self) -> EventType {
        EventType::MouseButtonReleased
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::MOUSE | EventCategory::INPUT
    }
}

impl fmt::Display for MouseButtonReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse Button Released Event: {}", self.mouse_button)
    }
}

/***********************************************************/
