use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TestError {
    #[error("Test failed")]
    Failed,
    #[error("Unsupported action {0}")]
    UnsupportedAction(String),
    #[error("Unsupported encoding {0}")]
    UnsupportedEncoding(String),
}
