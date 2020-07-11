use actix_web::{error::ResponseError, HttpResponse};
use deadpool_postgres::PoolError;
use rpel::error::RpelError;
use serde_json::error::Error as SJError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    // #[error("Internal Server Error")]
    // InternalServerError,
    #[error("Bad request: {0}")]
    BadRequest(String),

    // #[error("IO Error: {0}")]
    // IOError(std::io::Error),
    #[error("Unable to connect to the database")]
    PoolError(PoolError),

    #[error("DB Error: {0}")]
    DBError(RpelError),

    #[error("Serde JSON error: {0}")]
    SJError(SJError),
    // #[error("Not auth")]
    // NotAuth,
    // #[error("Authentication failed")]
    // FailedAuth,
    #[error("Error get client")]
    ClientGet,
}

impl From<RpelError> for ServiceError {
    fn from(error: RpelError) -> Self {
        Self::DBError(error)
    }
}

impl From<PoolError> for ServiceError {
    fn from(error: PoolError) -> Self {
        Self::PoolError(error)
    }
}

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
            ServiceError::DBError(_) => HttpResponse::BadRequest().reason("db error").finish(),
            ServiceError::SJError(_) => HttpResponse::BadRequest()
                .reason("serde json error")
                .finish(),
            // ServiceError::NotAuth => HttpResponse::BadRequest()
            //     .reason("Internal server error. Please try again later")
            //     .finish(),
            // ServiceError::FailedAuth => HttpResponse::BadRequest()
            //     .reason("Internal server error. Please try again later")
            //     .finish(),
            ServiceError::ClientGet => HttpResponse::BadRequest()
                .reason("Internal server error. Please try again later")
                .finish(),
        }
    }
}

impl From<SJError> for ServiceError {
    fn from(error: SJError) -> Self {
        Self::SJError(error)
    }
}
