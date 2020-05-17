use crate::events::event::Event;

pub struct WindowProperties {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

pub struct WindowData {
    pub width: u32,
    pub height: u32,
    pub is_vsync_enabled: bool,
}

pub trait Window {
    fn on_update(&mut self) -> ();
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_event_callback(&mut self, callback: fn(&dyn Event) -> ());
    fn set_vsync(&mut self, enabled: bool) -> ();
    fn is_vsync_enabled(&self) -> bool;
    fn create(window_props: WindowProperties) -> Self;
}
