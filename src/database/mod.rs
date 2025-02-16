use async_trait::async_trait;
use ordered_float::OrderedFloat;
use thiserror::Error;

pub mod postgres_connection;
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

#[derive(Debug)]
pub struct Candle {
    id: i32,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
}

impl Candle {
    pub fn new(id: i32, open: f64, close: f64, high: f64, low: f64) -> Candle {
        Candle {
            id,
            open,
            close,
            high,
            low,
        }
    }
    pub fn new_candle_from_tick(id: i32, all_tick: &[f64]) -> Candle {
        let filtered: Vec<_> = all_tick
            .iter()
            .filter(|&&x| !x.is_nan())
            .map(|&x| OrderedFloat(x))
            .collect();
        let max_value = filtered.iter().max();
        let min_value = filtered.iter().min();
        let max_value = max_value.map(|x| x.into_inner()).unwrap_or_default();
        let min_value = min_value.map(|x| x.into_inner()).unwrap_or_default();
        Candle::new(
            id,
            all_tick[0],
            all_tick[all_tick.len() - 1],
            max_value,
            min_value,
        )
    }
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
    /// Initiates a transactionnal writer to the State.
    async fn init_writer(&self) -> Result<StateTransaction, DatabaseError>;
    /// Reads an object by its candle_id, if any.
    ///
    /// # Arguments
    ///
    /// * `candle_id` - The `candle id` to requests.
    async fn read_candle_by_id(&self, candle_id: i32) -> Result<Option<Candle>, DatabaseError>;
}

/// Allow writing into the database.
#[async_trait]
pub trait StateWriter {
    /// Writes a new candle into the database.
    ///
    /// # Arguments
    ///
    /// * `candle` - A `Candle`, used to represent a duration on a stock market
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

#[async_trait]
impl StateWriter for StateTransaction<'_> {
    async fn write_candle(&mut self, candle: Candle) -> Result<(), DatabaseError> {
        match self {
            StateTransaction::Postgres(pg_tx) => pg_tx.write_candle(candle).await,
        }
    }
}
