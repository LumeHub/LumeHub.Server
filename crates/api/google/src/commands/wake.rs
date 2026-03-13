use serde::Deserialize;

use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;
use crate::error::GoogleCommandError;

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

    fn handle(
        &self,
        params: Self::Params,
        runtime: &dyn SceneRuntime,
    ) -> Result<(), GoogleCommandError> {
        runtime.start_wake(params.duration);
        Ok(())
    }
}
