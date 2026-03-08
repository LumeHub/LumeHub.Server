use serde::Deserialize;

use crate::lume_service::LumeService;

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

    fn handle(&self, _params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        lume_service.stop_light_effect();
        Ok(())
    }
}
