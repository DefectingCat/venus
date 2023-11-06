use serde::{Deserialize, Serialize};

pub enum RUAEvents {
    UpdateRuaConfig,
    UpdateCoreConfig,
    SpeedTest,
    EmitLog,
    UpdateUI,
}

impl RUAEvents {
    pub fn as_str(&self) -> &'static str {
        use RUAEvents::*;
        match self {
            UpdateRuaConfig => "rua://update-rua-config",
            UpdateCoreConfig => "rua://update-core-config",
            SpeedTest => "rua://speed-test",
            EmitLog => "rua://emit-log",
            UpdateUI => "rua://update-ui",
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UIPayload {
    pub main_show: bool,
}
