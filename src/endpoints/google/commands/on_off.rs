use serde::Deserialize;

use crate::lume_service::LumeService;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

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

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        lume_service.set_on_off(params.on);
        Ok(())
    }
}
