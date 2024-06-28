use super::{ConnectionError, DatabaseConnection, ProcessorError, RowDatabase};
use sqlite::{Connection, Value};
use thiserror::Error;
pub struct SQliteDatabase {
    connection: Connection,
}

#[derive(Debug, Error)]
pub enum SQLiteError {
    #[error(transparent)]
    SQLiteErrorConnection(#[from] sqlite::Error),
}

impl SQliteDatabase {
    pub fn try_new(path: &str) -> Result<Self, ConnectionError> {
        let connection = sqlite::open(path).map_err(SQLiteError::from)?;
        Ok(SQliteDatabase { connection })
    }
}

impl DatabaseConnection for SQliteDatabase {
    fn apply_query(&self, query: &str) -> Result<(), ProcessorError> {
        self.connection.execute(query).map_err(SQLiteError::from)?;
        Ok(())
    }

    fn read_query<T>(
        &self,
        query: &str,
        row_processor: fn(RowDatabase, kwargs: T) -> Result<T, ProcessorError>,
        mut kwargs: T,
    ) -> Result<T, ProcessorError> {
        for row in self
            .connection
            .prepare(query)
            .map_err(SQLiteError::from)?
            .into_iter()
        {
            kwargs = row_processor(
                RowDatabase::RowSQlite(&mut row.map_err(SQLiteError::from)?),
                kwargs,
            )?;
        }
        Ok(kwargs)
    }
}

pub fn extract_value_string(value: Value) -> String {
    let value_string: String = match value {
        Value::String(value_to_extract) => value_to_extract,
        Value::Float(value_to_extract) => value_to_extract.to_string(),
        Value::Null => "".to_string(),
        Value::Integer(value_to_extract) => value_to_extract.to_string(),
        Value::Binary(_) => todo!(),
    };
    value_string
}
