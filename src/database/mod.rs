mod postgres_connection;
mod sqlite_connection;

use sqlite::Row;
use sqlite_connection::SQLiteError;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error(transparent)]
    Sqlite(#[from] SQLiteError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error(transparent)]
    Sqlite(#[from] SQLiteError),
}

pub enum RowDatabase<'a> {
    RowSQlite(&'a mut Row),
}

pub trait DatabaseConnection {
    fn apply_query(&self, query: &str) -> Result<(), ProcessorError>;
    fn read_query<T>(
        &self,
        query: &str,
        row_processor: fn(RowDatabase, kwargs: T) -> Result<T, ProcessorError>,
        kwargs: T,
    ) -> Result<T, ProcessorError>;
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
