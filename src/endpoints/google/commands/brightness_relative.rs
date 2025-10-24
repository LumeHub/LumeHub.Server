use serde::Deserialize;

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
        let current_brightness = lume_service.lume_state.lock().unwrap().brightness as i16;
        let mut new_brightness = current_brightness;

        if let Some(percent) = params.brightness_relative_percent {
            new_brightness += percent as i16;
        } else if let Some(weight) = params.brightness_relative_weight {
            // Scale weight from -5 to 5 to a percentage change, e.g., -20% to 20%
            new_brightness += weight as i16 * 4; // 5 * 4 = 20%
        }

        lume_service.set_brightness(new_brightness.clamp(0, 100) as u8);
        Ok(())
    }
}
