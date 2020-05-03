use crate::blip_info;

pub trait Application {
    fn create_application() -> Box<Self>;

    fn run(&self) {
        blip_info!("BLIP application started!");
    }
}
