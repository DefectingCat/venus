use crate::consts::CORE_FOLDER;

pub struct VConfig {
    pub core_folder_name: &'static str,
    pub log_level: String,
}

impl VConfig {
    pub fn new() -> Self {
        Self {
            core_folder_name: CORE_FOLDER,
            log_level: "info".to_string(),
        }
    }
}
