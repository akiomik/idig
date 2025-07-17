use sea_orm::{Database, DatabaseConnection as SeaOrmConnection, DbErr};

pub struct DatabaseConnection {
    connection: SeaOrmConnection,
}

impl DatabaseConnection {
    /// Creates a new database connection
    ///
    /// # Errors
    ///
    /// Returns an error if the database connection fails
    #[inline]
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let connection = Database::connect(database_url).await?;
        Ok(Self { connection })
    }

    #[cfg(test)]
    #[must_use]
    #[inline]
    pub const fn new_from_connection(connection: SeaOrmConnection) -> Self {
        Self { connection }
    }

    #[must_use]
    #[inline]
    pub const fn get_connection(&self) -> &SeaOrmConnection {
        &self.connection
    }
}
