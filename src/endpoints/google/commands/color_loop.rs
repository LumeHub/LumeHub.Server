use serde::Deserialize;

use crate::lume_service::LumeService;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorLoopParams {
    pub duration: u64,
}

pub struct ColorLoopCommand;

impl GoogleCommandWithParams for ColorLoopCommand {
    type Params = ColorLoopParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::ColorLoop
    }

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        lume_service.start_color_loop(params.duration);
        Ok(())
    }
}
