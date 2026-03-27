use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid input")]
    BadRequest,
    #[error("internal error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "bad_request"),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        };
        (code, Json(json!({ "error": msg }))).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodecError {
    ValueOutOfRange { board_side: usize, pit: usize, value: u8 },
    InvalidTurn,
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::ValueOutOfRange { board_side, pit, value } => {
                write!(f, "pit value out of range at [{board_side}][{pit}]: {value}")
            }
            CodecError::InvalidTurn => write!(f, "Invalid turn value, expected 0 or 1"),
        }
    }
}

impl std::error::Error for CodecError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveError {
    InvalidMove,
    InvalidTurn
}