use blip::core::application::Application;

struct Sandbox {}

impl Application for Sandbox {
    fn createApplication() -> Box<Self> {
        Box::new(Sandbox {})
    }
}

fn main() {
    let app: Box<Sandbox> = Application::createApplication();
    app.run();
}
