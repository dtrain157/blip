use super::event::{Event, EventCategory, EventType};
use std::fmt;

/***********************************************************/

pub struct WindowResizeEvent {
    width: u16,
    height: u16,
    handled: bool,
}

impl Event for WindowResizeEvent {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> EventType {
        EventType::WindowResize
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }
}

impl fmt::Display for WindowResizeEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Window Resize Event: {} x {}", self.width, self.height)
    }
}

/***********************************************************/

pub struct WindowCloseEvent {
    handled: bool,
}

impl Event for WindowCloseEvent {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> EventType {
        EventType::WindowClose
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }
}

impl fmt::Display for WindowCloseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Window Close Event")
    }
}

/***********************************************************/

pub struct AppTickEvent {
    handled: bool,
}

impl Event for AppTickEvent {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> EventType {
        EventType::AppTick
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }
}

impl fmt::Display for AppTickEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App Tick Event")
    }
}

/***********************************************************/

pub struct AppUpdateEvent {
    handled: bool,
}

impl Event for AppUpdateEvent {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> EventType {
        EventType::AppUpdate
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }
}

impl fmt::Display for AppUpdateEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App Update Event")
    }
}

/***********************************************************/

pub struct AppRenderEvent {
    handled: bool,
}

impl Event for AppRenderEvent {
    fn is_handled(&self) -> bool {
        self.handled
    }

    fn get_event_type(&self) -> EventType {
        EventType::AppRender
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }
}

impl fmt::Display for AppRenderEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App Render Event")
    }
}

/***********************************************************/
