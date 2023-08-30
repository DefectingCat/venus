use std::{io, num::ParseIntError, sync::PoisonError};

use base64::DecodeError;
use serde::{Serialize, Serializer};
use tauri::api;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::message::ConfigMsg;

// #[allow(dead_code)]
#[derive(Error, Debug)]
pub enum VError {
    /// Reqwest error
    #[error("Failed to request: {0}")]
    RequestFaild(#[from] reqwest::Error),
    /// Base64 decode error
    #[error("Base64 decode error: {0}")]
    DecodeError(#[from] DecodeError),
    /// Json serialize error
    #[error("Serialize json error: {0}")]
    SerializeError(#[from] serde_json::Error),
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    /// Tauri API error
    #[error("API error: {0}")]
    ApiError(#[from] api::Error),
    ///
    #[error("Poison error: {0}")]
    PoisonError(String),
    /// Toml serialize error
    #[error("Serializer toml error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    /// Toml deserialize error
    #[error("Deserializer toml error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    /// Mpsc message error
    #[error("Send message failed: {0}")]
    MsgSendError(#[from] mpsc::error::SendError<ConfigMsg>),
    /// Tauri App error
    #[error("Tauri runtime error: {0}")]
    RunTimeError(#[from] tauri::Error),
    /// Target is None
    #[error("Target is empty: {0}")]
    EmptyError(&'static str),
    /// Some serve start error
    // #[error("Target init failed: {0}")]
    // InitError(&'static str),
    /// WindowError
    #[error("Cannot get taget window: {0}")]
    WindowError(&'static str),
    /// Convert int to string
    #[error("Failed to parse to int: {0}")]
    ParseIntError(#[from] ParseIntError),
    /// Common error
    #[error("Someting wrong: {0}")]
    CommonError(String),
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
