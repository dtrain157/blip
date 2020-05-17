pub mod core;
pub mod events;
pub mod platform;

use crate::{
    core::application::Application,
    core::window::{Window, WindowProperties},
    events::event::Event,
    platform::windows::windows_window::WindowsWindow,
};
use env_logger::{self, Env};
use std::io::Write;

pub fn start<T>(app: Box<T>)
where
    T: Application,
{
    init_log();
    blip_info!("Starting up Blip!");
    let mut window = WindowsWindow::new(WindowProperties {
        title: "Blip!",
        width: 800,
        height: 600,
    });
    window.set_event_callback(callback);
    app.init();
    let is_running = true;
    while is_running {
        window.on_update();
    }
}

fn callback(event: &dyn Event) {
    blip_info!(format!("Event! {} ", event));
}

fn init_log() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let ts = buf.timestamp();

            writeln!(buf, "{}: {}: {}", ts, record.level(), record.args())
        })
        .init();
}
