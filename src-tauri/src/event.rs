use serde::{Deserialize, Serialize};

pub enum RUAEvents {
    UpdateRuaConfig,
    UpdateCoreConfig,
    UpdateUI,
    SpeedTest,
    EmitLog,
}

impl RUAEvents {
    pub fn as_str(&self) -> &'static str {
        use RUAEvents::*;
        match self {
            UpdateRuaConfig => "rua://update-rua-config",
            UpdateCoreConfig => "rua://update-core-config",
            UpdateUI => "rua://update-ui",
            SpeedTest => "rua://speed-test",
            EmitLog => "rua://emit-log",
        }
    }
}

impl<'a> From<RUAEvents> for &'a str {
    fn from(val: RUAEvents) -> Self {
        val.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeedTestPayload<'a> {
    pub id: &'a str,
    pub loading: bool,
}
