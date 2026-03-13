use serde::Deserialize;

use crate::device::{google_to_internal_brightness, internal_to_google_brightness};
use application::SceneRuntime;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;
use crate::error::GoogleCommandError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightnessRelativeParams {
    brightness_relative_percent: Option<i8>,
    brightness_relative_weight: Option<i8>,
}

pub struct BrightnessRelativeCommand;

impl GoogleCommandWithParams for BrightnessRelativeCommand {
    type Params = BrightnessRelativeParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::BrightnessRelative
    }

    fn handle(
        &self,
        params: Self::Params,
        runtime: &dyn SceneRuntime,
    ) -> Result<(), GoogleCommandError> {
        let current_percent = internal_to_google_brightness(runtime.snapshot().brightness) as i16;
        let new_percent = if let Some(percent) = params.brightness_relative_percent {
            current_percent + percent as i16
        } else if let Some(weight) = params.brightness_relative_weight {
            current_percent + weight as i16 * 4
        } else {
            current_percent
        };

        runtime.set_brightness(google_to_internal_brightness(
            new_percent.clamp(0, 100) as u8
        ));
        Ok(())
    }
}
