use actix_web::{error::ResponseError, HttpResponse};
use deadpool_postgres::PoolError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    // #[error("Internal Server Error")]
    // InternalServerError,
    #[error("Bad request: {0}")]
    BadRequest(String),

    // #[error("IO Error: {0}")]
    // IOError(std::io::Error),
    #[error("Pool error: {0}")]
    PoolError(#[from] PoolError),

    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),

    #[error("Serde JSON error: {0}")]
    SJError(#[from] serde_json::error::Error),

    // #[error("Not auth")]
    // NotAuth,
    #[error("Authentication failed")]
    FailedAuth,
}

// impl From<RpelError> for ServiceError {
//     fn from(error: RpelError) -> Self {
//         Self::DBError(error)
//     }
// }

// impl From<PoolError> for ServiceError {
//     fn from(error: PoolError) -> Self {
//         Self::PoolError(error)
//     }
// }

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // ServiceError::InternalServerError => HttpResponse::BadRequest()
            //     .reason("Internal server error. Please try again later")
            //     .finish(),
            ServiceError::BadRequest(_) => {
                HttpResponse::BadRequest().reason("bad request").finish()
            }
            // ServiceError::IOError(_) => HttpResponse::BadRequest().reason("io error").finish(),
            ServiceError::PoolError(_) => HttpResponse::BadRequest()
                .reason("unable to connect to the database")
                .finish(),
            // ServiceError::DBError(_) => HttpResponse::BadRequest().reason("db error").finish(),
            ServiceError::SJError(_) => HttpResponse::BadRequest()
                .reason("serde json error")
                .finish(),
            // ServiceError::NotAuth => HttpResponse::BadRequest()
            //     .reason("Internal server error. Please try again later")
            //     .finish(),
            ServiceError::FailedAuth => HttpResponse::BadRequest()
                .reason("internal server error. please try again later")
                .finish(),
            ServiceError::DBQueryError(_) => HttpResponse::BadRequest().reason("db error").finish(),
        }
    }
}

// impl From<SJError> for ServiceError {
//     fn from(error: SJError) -> Self {
//         Self::SJError(error)
//     }
// }
