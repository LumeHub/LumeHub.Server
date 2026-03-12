#[derive(Clone, Debug, Default)]
pub enum BlendMode {
    #[default]
    Override,
    Add,
}

impl From<&str> for BlendMode {
    fn from(s: &str) -> Self {
        match s {
            "add" => BlendMode::Add,
            _ => BlendMode::Override,
        }
    }
}
