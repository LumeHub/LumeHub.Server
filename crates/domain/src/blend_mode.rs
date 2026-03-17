#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    #[default]
    Override,
    Add,
    Screen,
    Multiply,
}

impl From<&str> for BlendMode {
    fn from(s: &str) -> Self {
        match s {
            "add" => BlendMode::Add,
            "screen" => BlendMode::Screen,
            "multiply" => BlendMode::Multiply,
            _ => BlendMode::Override,
        }
    }
}
