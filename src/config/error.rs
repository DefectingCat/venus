use std::io;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("open file failed {0}")]
    File(#[from] io::Error),
    #[error("deserialize toml failed {0}")]
    DeserializeToml(#[from] toml::de::Error),
    #[error("serde json failed {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub type ConfigResult<T, E = ConfigError> = Result<T, E>;
