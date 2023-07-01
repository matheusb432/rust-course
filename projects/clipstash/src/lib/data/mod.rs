pub mod model;
pub mod query;

use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataErr {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
// NOTE DatabasePool is a pool of connections
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await
            .unwrap_or_else(|e| Self::handle_connection_error(e));
        Self(pool)
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }

    // NOTE `-> !` is the never type, indicates that the function panics
    fn handle_connection_error(e: sqlx::Error) -> ! {
        eprintln!("Error: {}", e);
        eprintln!("\nIf the database has not yet been created, run:\n $ sqlx database setup\n");
        panic!("database connection failed")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, From, Display)]
pub struct DbId(Uuid);
impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}
