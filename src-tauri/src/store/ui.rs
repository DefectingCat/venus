use serde::{Deserialize, Serialize};

/// 用于前端的全局状态
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UI {
    /// V2ray core status
    pub core_status: CoreStatus,
    /// V2ray core version
    pub core_version: String,
    /// The main window is visible
    pub main_visible: bool,
}

impl Default for UI {
    fn default() -> Self {
        use CoreStatus::*;
        UI {
            core_status: Stopped,
            core_version: String::new(),
            main_visible: true,
        }
    }
}

/// The core current status
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CoreStatus {
    Started,
    Restarting,
    Stopped,
}

impl CoreStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreStatus::Started => "Started",
            CoreStatus::Restarting => "Restarting",
            CoreStatus::Stopped => "Stopped",
        }
    }
}
