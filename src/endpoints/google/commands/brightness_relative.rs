use std::sync::MutexGuard;

use serde::Deserialize;

use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;

use super::super::request::CommandRequest;
use super::super::response::{CommandResponse, CommandStatus, DeviceStates};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightnessRelativeParams {
    brightness_relative_percent: Option<i8>,
    brightness_relative_weight: Option<i8>,
}

pub fn handle_brightness_relative_command(
    cmd_req: &CommandRequest,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
    make_error_responses: impl Fn(&str) -> Vec<CommandResponse>,
) -> Vec<CommandResponse> {
    serde_json::from_value::<BrightnessRelativeParams>(cmd_req.execution.first().unwrap().params.clone())
        .map(|params| {
            let mut new_brightness = state.brightness as i16;

            if let Some(percent) = params.brightness_relative_percent {
                new_brightness += percent as i16;
            } else if let Some(weight) = params.brightness_relative_weight {
                // Scale weight from -5 to 5 to a percentage change, e.g., -20% to 20%
                new_brightness += weight as i16 * 4; // 5 * 4 = 20%
            }

            state.brightness = new_brightness.clamp(0, 100) as u8;

            let effect = Box::new(FadeColor {
                color: state.active_color.with_brightness(state.brightness),
            });
            effect_queue.enqueue(effect);

            cmd_req
                .devices
                .iter()
                .map(|d| CommandResponse {
                    ids: vec![d.id.clone()],
                    status: CommandStatus::Success,
                    states: Some(DeviceStates {
                        on: Some(state.is_on),
                        online: Some(true),
                        brightness: Some(state.brightness),
                        color: None,
                    }),
                    error_code: None,
                })
                .collect()
        })
        .unwrap_or_else(|_| make_error_responses("badRequest"))
}