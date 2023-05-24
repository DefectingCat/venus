use std::io;

use base64::DecodeError;
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VError {
    #[error("Request error: {0}")]
    RequestFaild(#[from] reqwest::Error),

    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Serialize error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

// https://github.com/tauri-apps/tauri/discussions/3913
impl Serialize for VError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type VResult<T, E = VError> = anyhow::Result<T, E>;
