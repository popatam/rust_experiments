use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String),
    Validation(StatusCode, String),
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Sqlx(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    }
}

#[derive(Serialize)]
struct ApiError {
    error: &'static str,
    message: String,
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Sqlx(code, message) => {
                let body = Json(ApiError {
                    error: "db_error",
                    message: message,
                });
                (code, body).into_response()
            }

            Error::Validation(code, message) => {
                let body = Json(ApiError {
                    error: "validateion_error",
                    message: message,
                });
                (code, body).into_response()
            }

            Error::NotFound => {
                let body = Json(ApiError {
                    error: "not_found",
                    message: "Not found".to_string(),
                });
                (StatusCode::NOT_FOUND, body).into_response()
            }
        }
    }
}