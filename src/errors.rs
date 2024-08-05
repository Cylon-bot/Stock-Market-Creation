use crate::{database, trading_objects::MarketError, tools};

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
