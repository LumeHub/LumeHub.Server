use std::sync::MutexGuard;

use serde::Deserialize;

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;

use super::super::request::CommandRequest;
use super::super::response::{CommandResponse, CommandStatus, DeviceStates};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnOffParams {
    pub on: bool,
}

pub fn handle_on_off_command(
    cmd_req: &CommandRequest,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
    make_error_responses: impl Fn(&str) -> Vec<CommandResponse>,
) -> Vec<CommandResponse> {
    serde_json::from_value::<OnOffParams>(cmd_req.execution.first().unwrap().params.clone())
        .map(|params| {
            state.is_on = params.on;
            let effect = if params.on {
                Box::new(FadeColor {
                    color: state.active_color,
                })
            } else {
                Box::new(FadeColor { color: Rgb::BLACK })
            };
            effect_queue.enqueue(effect);

            cmd_req
                .devices
                .iter()
                .map(|d| CommandResponse {
                    ids: vec![d.id.clone()],
                    status: CommandStatus::Success,
                    states: Some(DeviceStates {
                        on: Some(params.on),
                        online: Some(true),
                        color: None,
                    }),
                    error_code: None,
                })
                .collect()
        })
        .unwrap_or_else(|_| make_error_responses("badRequest"))
}
