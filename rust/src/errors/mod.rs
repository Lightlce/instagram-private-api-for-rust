use thiserror::Error;

/// Typed error hierarchy for client/network/protocol/domain failures.
#[derive(Debug, Error)]
pub enum IgApiError {
    #[error("client error: {0}")]
    Client(String),
}
