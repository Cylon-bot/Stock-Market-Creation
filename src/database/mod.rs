use async_trait::async_trait;
use thiserror::Error;

mod postgres_connection;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Postgres(#[from] postgres_connection::PgError),
}
#[must_use]
pub enum StateTransaction<'pg> {
    /// Postgres implementation.
    Postgres(postgres_connection::PgTransaction<'pg>),
}
pub struct Candle {
    id: i32,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
}
#[async_trait]
pub trait Transaction {
    /// Commits the operations made in the transaction. Effectively writing them.
    async fn commit(self) -> Result<(), DatabaseError>;

    /// Rollbacks the operations made in the transaction. Cancels every writes done.
    async fn rollback(self) -> Result<(), DatabaseError>;
}
/// Allow reading into the database.
#[async_trait]
pub trait StateReader: Send + Sync {
    /// Reads an object by its Cid, if any.
    ///
    /// # Arguments
    ///
    /// * `candle_id` - The `candle id` to requests.
    async fn read_candle_by_id(&self, candle_id: usize) -> Result<Option<Candle>, DatabaseError>;
}

/// Allow writing into the database.
#[async_trait]
pub trait StateWriter {
    /// Writes a new candle into the database.
    ///
    /// # Arguments
    ///
    /// * `record_input` - A `RecordInput`, used to populate the record's data.
    async fn write_candle(&mut self, candle: Candle) -> Result<(), DatabaseError>;
}

#[async_trait]
impl Transaction for StateTransaction<'_> {
    async fn commit(self) -> Result<(), DatabaseError> {
        match self {
            StateTransaction::Postgres(tx) => tx.commit().await,
        }
    }

    async fn rollback(self) -> Result<(), DatabaseError> {
        match self {
            StateTransaction::Postgres(tx) => tx.rollback().await,
        }
    }
}
