use crate::{database, tools};

use std::env::{self, VarError};
use thiserror::Error;

use database::DatabaseError;

#[derive(Debug, Error)]
pub enum MainProcessError {
    #[error(transparent)]
    EnvError(#[from] VarError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ConfError(#[from] tools::yaml_connection::YamlError),
}
