//! Error types for Systematics API

use std::fmt;

/// API error type for client-side error handling
#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
