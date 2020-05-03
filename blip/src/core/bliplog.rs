#[macro_export]
macro_rules! blip_info {
    ($name:expr) => {
        log::info!("[BLIP CORE] {}", $name);
    };
}

#[macro_export]
macro_rules! blip_warn {
    ($name:expr) => {
        log::warn!("[BLIP CORE] {}", $name);
    };
}

#[macro_export]
macro_rules! blip_error {
    ($name:expr) => {
        log::error!("[BLIP CORE] {}", $name);
    };
}

#[macro_export]
macro_rules! blip_debug {
    ($name:expr) => {
        log::debug!("[BLIP CORE] {}", $name);
    };
}
