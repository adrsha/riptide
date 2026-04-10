use thiserror::Error;

pub type RTResult<T> = Result<T, RTErrors>;

#[derive(Debug, Error)]
pub enum RTErrors {
    #[error("not found: {key}")]
    NotFound { key: String },

    #[error("already exists: {key}")]
    AlreadyExists { key: String },

    #[error("out of bounds: index={index}, len={len}")]
    OutOfBounds { index: usize, len: usize },

    #[error("type mismatch: expected={expected}, got={got}")]
    TypeMismatch { expected: String, got: String },

    #[error("null or missing value: {field}")]
    NullValue { field: String },

    #[error("overflow: {context}")]
    Overflow { context: String },

    #[error("underflow: {context}")]
    Underflow { context: String },

    #[error("invalid value: {field} — {reason}")]
    InvalidValue { field: String, reason: String },

    #[error("parse error: {context} — {reason}")]
    Parse { context: String, reason: String },

    #[error("encoding error: {context} — {reason}")]
    Encoding { context: String, reason: String },

    #[error("deserialization error: {context} — {reason}")]
    Deserialize { context: String, reason: String },

    #[error("serialization error: {context} — {reason}")]
    Serialize { context: String, reason: String },

    #[error("stack overflow: {context}")]
    StackOverflow { context: String },

    #[error("deadlock detected: {context}")]
    Deadlock { context: String },

    #[error("timeout: {context} exceeded {limit_ms}ms")]
    Timeout { context: String, limit_ms: u64 },

    #[error("cancelled: {context}")]
    Cancelled { context: String },

    #[error("exhausted: {resource}")]
    Exhausted { resource: String },

    #[error("unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("forbidden: {reason}")]
    Forbidden { reason: String },

    #[error("io error: {source}")]
    Io {
        #[from]
        source: std::io::Error
    },

    #[error("network error: {message}")]
    Network {
        message: String,
        #[source]
        source:  Option<Box<dyn std::error::Error + Send + Sync>>
    },

    #[error("invalid state: {context} — {reason}")]
    InvalidState { context: String, reason: String },

    #[error("uninitialized: {component}")]
    Uninitialized { component: String },

    #[error("poisoned: {context}")]
    Poisoned { context: String },

    #[error("external error: {message}")]
    External {
        message: String,
        #[source]
        source:  Box<dyn std::error::Error + Send + Sync>
    },

    #[error("internal error: {message}")]
    Internal { message: String }
}
