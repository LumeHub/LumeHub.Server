use thiserror::Error;

#[derive(Debug, Error)]
pub enum EffectError {
    #[error("unknown layer effect: {0}")]
    UnknownEffect(String),

    #[error("invalid params for '{effect}': {source}")]
    InvalidParams {
        effect: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("script compile error: {0}")]
    ScriptCompile(#[source] rhai::ParseError),

    #[error("error building layer '{effect}': {source}")]
    LayerBuild {
        effect: String,
        #[source]
        source: Box<EffectError>,
    },

    #[error("preset signal '{signal}' script error: {source}")]
    SignalScript {
        signal: String,
        #[source]
        source: Box<EffectError>,
    },

    #[error("preset nesting too deep (max {0})")]
    NestingTooDeep(usize),
}
