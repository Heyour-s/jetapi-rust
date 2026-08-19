use thiserror::Error;

pub type Result<T> = std::result::Result<T, JetError>;

#[derive(Error, Debug)]
pub enum JetError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
}

impl From<poem::Error> for JetError {
    fn from(err: poem::Error) -> Self {
        JetError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for JetError {
    fn from(err: serde_json::Error) -> Self {
        JetError::BadRequest(err.to_string())
    }
}

impl From<anyhow::Error> for JetError {
    fn from(err: anyhow::Error) -> Self {
        JetError::Internal(err.to_string())
    }
}