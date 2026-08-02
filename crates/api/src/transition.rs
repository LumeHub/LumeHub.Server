use application::SceneRuntime;
use domain::{TransitionCurve, TransitionSpec};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct TransitionQuery {
    pub fade_ms: Option<u32>,
    pub curve: Option<TransitionCurve>,
}

impl TransitionQuery {
    pub fn resolve(self, runtime: &dyn SceneRuntime) -> TransitionSpec {
        let default = runtime.default_spec();
        TransitionSpec::new(
            self.fade_ms.unwrap_or(default.duration_ms),
            self.curve.unwrap_or(default.curve),
        )
    }
}
