use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    TTError(tokio_tungstenite::tungstenite::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Unable to connect to the database")]
    PoolError(deadpool_postgres::PoolError),
    #[error("Serde JSON error: {0}")]
    SJError(serde_json::error::Error),
    #[error("Not auth")]
    NotAuth,
    #[error("Not permission")]
    NotPermission,
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
}

impl From<deadpool_postgres::PoolError> for ServiceError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        Self::PoolError(error)
    }
}

impl From<serde_json::error::Error> for ServiceError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::SJError(error)
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
impl From<tokio_tungstenite::tungstenite::Error> for ServiceError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::TTError(error)
    }
}
