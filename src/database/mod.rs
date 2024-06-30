use async_trait::async_trait;
use postgres_connection::PgError;

mod postgres_connection;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error(transparent)]
    Sqlite(#[from] PgError),
}

#[async_trait]
pub trait Transaction {
    /// Commits the operations made in the transaction. Effectively writing them.
    async fn commit(self) -> Result<(), StateError>;

    /// Rollbacks the operations made in the transaction. Cancels every writes done.
    async fn rollback(self) -> Result<(), StateError>;
}

#[async_trait]
impl Transaction for StateTransaction<'_> {
    async fn commit(self) -> Result<(), StateError> {
        match self {
            StateTransaction::Postgres(tx) => tx.commit().await,
        }
    }

    async fn rollback(self) -> Result<(), StateError> {
        match self {
            StateTransaction::Postgres(tx) => tx.rollback().await,
        }
    }
}

#[must_use]
pub enum StateTransaction<'pg> {
    /// Postgres implementation.
    Postgres(postgres::PgTransaction<'pg>),
}
