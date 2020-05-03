use blip::core::application::Application;
use env_logger::{self, Env};
use std::io::Write;

struct Sandbox {}

impl Application for Sandbox {
    fn create_application() -> Box<Self> {
        Box::new(Sandbox {})
    }
}

fn main() {
    init_log();
    let app: Box<Sandbox> = Application::create_application();
    app.run();
}

fn init_log() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let ts = buf.timestamp();

            writeln!(buf, "{}: {}: {}", ts, record.level(), record.args())
        })
        .init();
}
