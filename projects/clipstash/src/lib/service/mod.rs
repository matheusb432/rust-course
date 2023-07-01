pub mod action;
pub mod ask;

use crate::{ClipErr, DataErr};

#[derive(Debug, thiserror::Error)]
pub enum ServiceErr {
    #[error("clip error: {0}")]
    Clip(#[from] ClipErr),
    #[error("database error: {0}")]
    Data(DataErr),
    #[error("not found")]
    NotFound,
    #[error("permissions not met: {0}")]
    PermissionErr(String),
}

impl From<DataErr> for ServiceErr {
    fn from(err: DataErr) -> Self {
        match err {
            DataErr::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataErr::Database(other)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceErr {
    // TODO refactor to be used in From<DataErr> impl?
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataErr::Database(other)),
        }
    }
}
