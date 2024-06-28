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

pub trait DatabaseConnection {
    fn apply_query(&self, query: &str) -> Result<(), ProcessorError>;
    fn read_query<T>(
        &self,
        query: &str,
        row_processor: fn(RowDatabase, kwargs: T) -> Result<T, ProcessorError>,
        kwargs: T,
    ) -> Result<T, ProcessorError>;
}

pub enum RowDatabase<'a> {
    RowSQlite(&'a mut Row),
}
