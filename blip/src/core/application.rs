pub trait Application {
    fn createApplication() -> Box<Self>;

    fn run(&self) {
        while (true) {}
    }
}
