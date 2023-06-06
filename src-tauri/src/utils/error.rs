use std::{io, sync::PoisonError};

use base64::DecodeError;
use serde::{Serialize, Serializer};
use tauri::api;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::message::ConfigMsg;

#[derive(Error, Debug)]
pub enum VError {
    #[error("Failed to request: {0}")]
    RequestFaild(#[from] reqwest::Error),

    #[error("Base64 decode error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Serialize json error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("API error: {0}")]
    ApiError(#[from] api::Error),

    #[error("Poison error: {0}")]
    PoisonError(String),

    #[error("Serializer toml error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("Deserializer toml error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("Send message failed: {0}")]
    MsgSendError(#[from] mpsc::error::SendError<ConfigMsg>),
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

impl<T> From<PoisonError<T>> for VError {
    fn from(value: PoisonError<T>) -> Self {
        Self::PoisonError(value.to_string())
    }
}

pub type VResult<T, E = VError> = anyhow::Result<T, E>;