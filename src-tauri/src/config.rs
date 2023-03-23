use crate::consts::CORE_FOLDER;

pub struct VConfig {
    pub log_level: String,
}

impl VConfig {
    pub fn new() -> Self {
        Self {
            log_level: "info".to_string(),
        }
    }
}
