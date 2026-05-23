use serde::{Deserialize, Serialize};

use super::curve::TransitionCurve;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSpec {
    pub duration_ms: u32,
    #[serde(default)]
    pub curve: TransitionCurve,
}

impl TransitionSpec {
    pub const INSTANT: Self = Self {
        duration_ms: 0,
        curve: TransitionCurve::Linear,
    };

    pub fn new(duration_ms: u32, curve: TransitionCurve) -> Self {
        Self { duration_ms, curve }
    }

    pub fn frames(self, frame_ms: u32) -> u32 {
        match frame_ms {
            0 => 0,
            _ => self.duration_ms / frame_ms,
        }
    }

    pub fn is_instant(self) -> bool {
        self.duration_ms == 0
    }
}

impl Default for TransitionSpec {
    fn default() -> Self {
        Self::INSTANT
    }
}
