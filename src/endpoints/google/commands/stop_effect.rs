use serde::Deserialize;

use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopEffectParams {}

pub struct StopEffectCommand;

impl GoogleCommandWithParams for StopEffectCommand {
    type Params = StopEffectParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::StopEffect
    }

    fn handle(&self, _params: Self::Params, runtime: &dyn SceneRuntime) -> Result<(), String> {
        runtime.stop_effect();
        Ok(())
    }
}
