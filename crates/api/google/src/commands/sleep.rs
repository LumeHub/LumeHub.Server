use serde::Deserialize;

use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;
use crate::error::GoogleCommandError;

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

    fn handle(
        &self,
        params: Self::Params,
        runtime: &dyn SceneRuntime,
    ) -> Result<(), GoogleCommandError> {
        runtime.start_sleep(params.duration);
        Ok(())
    }
}
