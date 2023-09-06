use serde::{Deserialize, Serialize};

pub enum RUAEvents {
    UpdateRuaConfig,
    UpdateCoreConfig,
    SpeedTest,
}

impl RUAEvents {
    pub fn as_str(&self) -> &str {
        use RUAEvents::*;

        match self {
            UpdateRuaConfig => "rua://update-rua-config",
            UpdateCoreConfig => "rua://update-core-config",
            SpeedTest => "rua://speed-test",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeedTestPayload<'a> {
    pub id: &'a str,
    pub loading: bool,
}
