use serde::Deserialize;

use crate::endpoints::google::device::google_to_internal_brightness;
use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightnessAbsoluteParams {
    pub brightness: u8,
}

pub struct BrightnessAbsoluteCommand;

impl GoogleCommandWithParams for BrightnessAbsoluteCommand {
    type Params = BrightnessAbsoluteParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::BrightnessAbsolute
    }

    fn handle(&self, params: Self::Params, runtime: &dyn SceneRuntime) -> Result<(), String> {
        runtime.set_brightness(google_to_internal_brightness(params.brightness));
        Ok(())
    }
}
