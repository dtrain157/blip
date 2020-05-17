use blip;
use blip::core::application::Application;

struct Sandbox {}

impl Application for Sandbox {
    fn init(&self) {}
}

fn main() {
    blip::start(Box::new(Sandbox {}))
}
