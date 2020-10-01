use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    // #[error("Null error")]
    // NullError,
    #[error("Socket address parse: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Unable to connect to the database: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Serde JSON error: {0}")]
    SJError(#[from] serde_json::error::Error),
    #[error("Not auth")]
    NotAuth,
    #[error("Not permission")]
    NotPermission,
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("Warp error: {0}")]
    WarpError(#[from] warp::Error),
}

// impl From<()> for ServiceError {
//     fn from(_error: ()) -> Self {
//         Self::NullError
//     }
// }
