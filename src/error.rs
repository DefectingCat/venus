use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {}

pub type AppResult<T, E = AppError> = Result<T, E>;
