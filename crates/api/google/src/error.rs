use thiserror::Error;

/// Google Smart Home protocol error codes.
/// The `Display` impl (via `#[error]`) produces the exact string Google expects.
#[derive(Debug, Error)]
pub enum GoogleCommandError {
    #[error("badRequest")]
    BadRequest,

    #[error("noColorProvided")]
    NoColorProvided,
}
