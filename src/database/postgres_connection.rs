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
    async fn commit(self) -> Result<(), StateError> {
        self.0.commit().await.map_err(PgError::from)?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), StateError> {
        self.0.rollback().await.map_err(PgError::from)?;
        Ok(())
    }
}
