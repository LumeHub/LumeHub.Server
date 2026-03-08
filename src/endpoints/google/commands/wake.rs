use serde::Deserialize;

use crate::lume_service::LumeService;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WakeParams {
    pub duration: u64,
}

pub struct WakeCommand;

impl GoogleCommandWithParams for WakeCommand {
    type Params = WakeParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::Wake
    }

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        lume_service.start_wake_effect(params.duration);
        Ok(())
    }
}
