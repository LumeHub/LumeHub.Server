use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignalError {
    #[error("script compile error: {0}")]
    ScriptCompile(String),
}
