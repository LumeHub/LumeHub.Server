use serde::Deserialize;

use crate::lume_service::LumeService;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepParams {
    pub duration: u64,
}

pub struct SleepCommand;

impl GoogleCommandWithParams for SleepCommand {
    type Params = SleepParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::Sleep
    }

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        lume_service.start_sleep_effect(params.duration);
        Ok(())
    }
}
