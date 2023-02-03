use anyhow::{Error, Result};
use serde::Serialize;
use std::env;
use std::path::PathBuf;

struct RUAError(Error);

// we must manually implement serde::Serialize
impl serde::Serialize for RUAError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_ref())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn current_dir() -> Result<PathBuf, String> {
    Ok(env::current_dir().expect("error while read current dir"))
}
