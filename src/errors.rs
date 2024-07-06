use crate::database;

use std::env::{self, VarError};
use thiserror::Error;

use database::DatabaseError;

#[derive(Debug, Error)]
pub enum MainProcessError {
    #[error(transparent)]
    EnvError(#[from] VarError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
