use thiserror::Error;

/// Typed error hierarchy for client/network/protocol/domain failures.
#[derive(Debug, Error)]
pub enum IgApiError {
    #[error("client error: {0}")]
    Client(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("signature error: {0}")]
    Signature(String),
    #[error("http status {status}: {body}")]
    HttpResponse { status: u16, body: String },
}
