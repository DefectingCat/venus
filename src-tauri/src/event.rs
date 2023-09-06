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
