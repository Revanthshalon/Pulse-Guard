#![allow(unused)]

use axum::{http::StatusCode, response::IntoResponse};
use tracing::error;

#[derive(Debug)]
pub enum AccessGridErrors {
    Configuration(String),
    Database(sqlx::error::Error),
}

impl std::fmt::Display for AccessGridErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessGridErrors::Database(e) => write!(f, "Database Error: {:?}", e),
            AccessGridErrors::Configuration(e) => write!(f, "Configuration Error: {:?}", e),
        }
    }
}

impl std::error::Error for AccessGridErrors {}

impl From<sqlx::error::Error> for AccessGridErrors {
    fn from(value: sqlx::error::Error) -> Self {
        AccessGridErrors::Database(value)
    }
}

pub type AccessGridResult<T> = Result<T, AccessGridErrors>;

impl IntoResponse for AccessGridErrors {
    fn into_response(self) -> axum::response::Response {
        error!("{self:#?}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
