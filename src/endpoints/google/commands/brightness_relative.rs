use serde::Deserialize;

use crate::endpoints::google::device::{
    google_to_internal_brightness, internal_to_google_brightness,
};
use crate::lume_service::LumeService;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

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

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        let current_percent =
            internal_to_google_brightness(lume_service.lume_state.lock().unwrap().brightness)
                as i16;
        let new_percent = if let Some(percent) = params.brightness_relative_percent {
            current_percent + percent as i16
        } else if let Some(weight) = params.brightness_relative_weight {
            current_percent + weight as i16 * 4
        } else {
            current_percent
        };

        lume_service.set_brightness(google_to_internal_brightness(
            new_percent.clamp(0, 100) as u8
        ));
        Ok(())
    }
}
