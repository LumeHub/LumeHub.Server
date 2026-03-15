use std::sync::Arc;

use application::SceneSnapshot;
use domain::Rgb;
use effects::LiveParam;
use persistence::PersistedState;

pub(crate) enum ActiveScene {
    Idle,
    Running {
        brightness: Arc<LiveParam<f32>>,
    },
    Timed {
        name: String,
        end_unix_timestamp_sec: u64,
    },
}

pub(crate) struct SceneState {
    pub(crate) on: bool,
    pub(crate) brightness: u8,
    pub(crate) color: Rgb,
    pub(crate) scene: ActiveScene,
}

pub(crate) fn build_snapshot(state: &SceneState) -> SceneSnapshot {
    let (active_effect, light_effect_end_unix_timestamp_sec) = match &state.scene {
        ActiveScene::Idle => (None, None),
        ActiveScene::Running { .. } => (Some("custom".to_string()), None),
        ActiveScene::Timed {
            name,
            end_unix_timestamp_sec,
        } => (Some(name.clone()), Some(*end_unix_timestamp_sec)),
    };
    SceneSnapshot {
        on: state.on,
        brightness: state.brightness,
        color: state.color,
        active_effect,
        light_effect_end_unix_timestamp_sec,
    }
}

pub(crate) fn extract_persisted(state: &SceneState) -> PersistedState {
    PersistedState {
        on: state.on,
        brightness: state.brightness,
        color: state.color,
        active_effect: match &state.scene {
            ActiveScene::Idle => None,
            ActiveScene::Running { .. } => Some("custom".to_string()),
            ActiveScene::Timed { name, .. } => Some(name.clone()),
        },
    }
}
