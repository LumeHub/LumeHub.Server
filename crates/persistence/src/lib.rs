use std::collections::HashMap;
use std::path::Path;

use domain::Rgb;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub on: bool,
    pub brightness: u8,
    pub color: Rgb,
    #[serde(default)]
    pub active_effect: Option<String>,
    #[serde(default)]
    pub signal_colors: HashMap<String, Rgb>,
    #[serde(default)]
    pub signal_scripts: HashMap<String, String>,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            on: true,
            brightness: 255,
            color: Rgb::BLACK,
            active_effect: None,
            signal_colors: HashMap::new(),
            signal_scripts: HashMap::new(),
        }
    }
}

pub fn load(path: &Path) -> Result<PersistedState, PersistenceError> {
    match std::fs::read_to_string(path) {
        Ok(json) => Ok(serde_json::from_str(&json)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PersistedState::default()),
        Err(e) => Err(PersistenceError::Io(e)),
    }
}

pub fn save(path: &Path, state: &PersistedState) -> Result<(), PersistenceError> {
    let json = serde_json::to_string(state)?;
    std::fs::write(path, json)?;
    Ok(())
}
