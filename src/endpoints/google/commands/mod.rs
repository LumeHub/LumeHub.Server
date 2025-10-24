pub mod color_absolute;
pub mod on_off;
pub mod brightness_absolute;
pub mod brightness_relative;

use std::sync::MutexGuard;

use crate::effects::EffectQueue;
use crate::state::LumeState;

use super::request::{CommandRequest, ExecuteCommandType};
use super::response::CommandResponse;

pub fn process_command(
    cmd_req: &CommandRequest,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
) -> Vec<CommandResponse> {
    let make_error_responses = |error_code: &str| {
        cmd_req
            .devices
            .iter()
            .map(|d| CommandResponse {
                ids: vec![d.id.clone()],
                status: super::response::CommandStatus::Error,
                error_code: Some(error_code.to_string()),
                states: None,
            })
            .collect()
    };

    let Some(exec_req) = cmd_req.execution.first() else {
        return make_error_responses("badRequest");
    };

    match exec_req.command {
        ExecuteCommandType::OnOff => {
            on_off::handle_on_off_command(cmd_req, state, effect_queue, make_error_responses)
        }
        ExecuteCommandType::ColorAbsolute => color_absolute::handle_color_absolute_command(
            cmd_req,
            state,
            effect_queue,
            make_error_responses,
        ),
        ExecuteCommandType::BrightnessAbsolute => brightness_absolute::handle_brightness_absolute_command(
            cmd_req,
            state,
            effect_queue,
            make_error_responses,
        ),
        ExecuteCommandType::BrightnessRelative => brightness_relative::handle_brightness_relative_command(
            cmd_req,
            state,
            effect_queue,
            make_error_responses,
        ),
    }
}
