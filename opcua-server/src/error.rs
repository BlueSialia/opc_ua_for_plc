//! Error types for the opcua-server crate.

use thiserror::Error;

/// Errors surfaced by the opcua-server crate.
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("OPC UA backend error: {0}")]
    Backend(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Other: {0}")]
    Other(String),
}
