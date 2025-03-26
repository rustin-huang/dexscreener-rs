use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error response from the DexScreener API.
///
/// This struct represents the error information returned by the API
/// when a request fails.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code provided by the API
    pub code: Option<String>,
    /// Human-readable error message
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DexScreener API error: {} (code: {})",
            self.message,
            self.code.as_deref().unwrap_or("unknown")
        )
    }
}

/// Errors that can occur when interacting with the DexScreener API.
///
/// This enum represents the various error conditions that may arise
/// when using this library, including network errors, API errors,
/// and parsing errors.
#[derive(Error, Debug)]
pub enum DexScreenerError {
    /// An error occurred during the HTTP request
    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// The API returned an error response
    #[error("API error: {0:?}")]
    ApiError(ErrorResponse),

    /// Failed to parse the JSON response
    #[error("JSON parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Other miscellaneous errors
    #[error("Other error: {0}")]
    Other(String),
}

impl DexScreenerError {
    /// Creates a new custom error with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A description of the error
    ///
    /// # Examples
    ///
    /// ```
    /// use dexscreener_rs::DexScreenerError;
    ///
    /// let error = DexScreenerError::new("Invalid input parameter");
    /// ```
    pub fn new<S: Into<String>>(message: S) -> Self {
        DexScreenerError::Other(message.into())
    }
}
