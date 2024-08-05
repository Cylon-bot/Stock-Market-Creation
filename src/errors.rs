use crate::{database, tools, trading_objects::MarketError};

use std::env::VarError;
use thiserror::Error;

use database::DatabaseError;

#[derive(Debug, Error)]
pub enum MainProcessError {
    #[error(transparent)]
    Env(#[from] VarError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Conf(#[from] tools::yaml_connection::YamlError),
    #[error(transparent)]
    MarketError(#[from] MarketError),
}
