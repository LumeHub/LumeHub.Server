use serde::Deserialize;

use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;
use crate::error::GoogleCommandError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnOffParams {
    pub on: bool,
}

pub struct OnOffCommand;

impl GoogleCommandWithParams for OnOffCommand {
    type Params = OnOffParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::OnOff
    }

    fn handle(
        &self,
        params: Self::Params,
        runtime: &dyn SceneRuntime,
    ) -> Result<(), GoogleCommandError> {
        runtime.set_on_off(params.on);
        Ok(())
    }
}
