use std::sync::MutexGuard;

use serde::Deserialize;

use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;

use super::super::request::CommandRequest;
use super::super::response::{CommandResponse, CommandStatus, DeviceStates};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightnessAbsoluteParams {
    pub brightness: u8,
}

pub fn handle_brightness_absolute_command(
    cmd_req: &CommandRequest,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
    make_error_responses: impl Fn(&str) -> Vec<CommandResponse>,
) -> Vec<CommandResponse> {
    serde_json::from_value::<BrightnessAbsoluteParams>(cmd_req.execution.first().unwrap().params.clone())
        .map(|params| {
            state.brightness = params.brightness;
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