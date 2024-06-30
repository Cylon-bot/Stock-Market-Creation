use super::{Candle, DatabaseError, StateTransaction, StateWriter, Transaction};
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use thiserror::Error;

/// Postgres implementation errors.
#[derive(Debug, Error)]
pub enum PgError {
    /// Error raised while querying with Sqlx.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

/// A Postgres implementation of a State.
pub struct PgState {
    /// Sqlx `Pool` used to interact with the database.
    pool: Pool<Postgres>,
}

impl PgState {
    /// Tries to instanciate a new `PgState` from a given psql connection url.
    ///
    /// # Arguments
    ///
    /// * `url` - A postgres connection url to try to connect to.
    pub async fn try_new(url: &str) -> Result<Self, PgError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .map_err(PgError::from)?;

        Ok(Self { pool })
    }
}

pub struct PgTransaction<'tx>(sqlx::Transaction<'tx, Postgres>);

#[async_trait]
impl Transaction for PgTransaction<'_> {
    async fn commit(self) -> Result<(), DatabaseError> {
        self.0.commit().await.map_err(PgError::from)?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), DatabaseError> {
        self.0.rollback().await.map_err(PgError::from)?;
        Ok(())
    }
}
#[async_trait]
impl StateWriter for PgTransaction<'_> {
    async fn write_candle(&mut self, candle: Candle) -> Result<(), DatabaseError> {
        sqlx::query!(
            r#"
                INSERT INTO M1 (candle_id, open, close, high, low)
                VALUES ($1, $2, $3, $4, $5);
            "#,
            candle.id,
            candle.open,
            candle.close,
            candle.high,
            candle.low,
        )
        .fetch_one(&mut *self.0)
        .await
        .map_err(PgError::from)?;

        Ok(())
    }
}
