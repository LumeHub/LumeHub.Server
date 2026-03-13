pub mod brightness_absolute;
pub mod brightness_relative;
pub mod color_absolute;
pub mod color_loop;
pub mod on_off;
pub mod sleep;
pub mod stop_effect;
pub mod wake;

use application::SceneRuntime;
use serde::de::DeserializeOwned;

use super::request::{CommandRequest, ExecuteCommandType};
use super::response::{CommandResponse, CommandStatus};
use crate::device::device_states_from_snapshot;

pub trait GoogleCommand: Send + Sync {
    fn command_type(&self) -> ExecuteCommandType;
    fn handle(&self, cmd_req: &CommandRequest, runtime: &dyn SceneRuntime) -> Result<(), String>;
}

pub trait GoogleCommandWithParams: Send + Sync {
    type Params: DeserializeOwned;
    fn command_type(&self) -> ExecuteCommandType;
    fn handle(&self, params: Self::Params, runtime: &dyn SceneRuntime) -> Result<(), String>;
}

impl<T> GoogleCommand for T
where
    T: GoogleCommandWithParams + Sized + 'static,
{
    fn command_type(&self) -> ExecuteCommandType {
        <Self as GoogleCommandWithParams>::command_type(self)
    }

    fn handle(&self, cmd_req: &CommandRequest, runtime: &dyn SceneRuntime) -> Result<(), String> {
        cmd_req
            .get_params::<T::Params>()
            .and_then(|params| self.handle(params, runtime))
    }
}

pub struct CommandDispatcher {
    commands: Vec<Box<dyn GoogleCommand>>,
}

impl Clone for CommandDispatcher {
    fn clone(&self) -> Self {
        CommandDispatcher::new()
    }
}

impl CommandDispatcher {
    pub fn new() -> Self {
        CommandDispatcher {
            commands: vec![
                Box::new(on_off::OnOffCommand),
                Box::new(color_absolute::ColorAbsoluteCommand),
                Box::new(brightness_absolute::BrightnessAbsoluteCommand),
                Box::new(brightness_relative::BrightnessRelativeCommand),
                Box::new(color_loop::ColorLoopCommand),
                Box::new(sleep::SleepCommand),
                Box::new(wake::WakeCommand),
                Box::new(stop_effect::StopEffectCommand),
            ],
        }
    }

    pub fn process_command(
        &self,
        cmd_req: &CommandRequest,
        runtime: &dyn SceneRuntime,
    ) -> Vec<CommandResponse> {
        let make_error_responses = |error_code: &str| {
            cmd_req
                .devices
                .iter()
                .map(|d| CommandResponse {
                    ids: vec![d.id.clone()],
                    status: CommandStatus::Error,
                    error_code: Some(error_code.to_string()),
                    states: None,
                })
                .collect()
        };

        let Some(exec_req) = cmd_req.execution.first() else {
            return make_error_responses("badRequest");
        };

        if let Some(command_handler) = self
            .commands
            .iter()
            .find(|cmd| cmd.command_type() == exec_req.command)
        {
            match command_handler.handle(cmd_req, runtime) {
                Ok(_) => cmd_req
                    .devices
                    .iter()
                    .map(|d| CommandResponse {
                        ids: vec![d.id.clone()],
                        status: CommandStatus::Success,
                        states: Some(device_states_from_snapshot(&runtime.snapshot())),
                        error_code: None,
                    })
                    .collect(),
                Err(e) => make_error_responses(&e),
            }
        } else {
            make_error_responses("unsupportedCommand")
        }
    }
}
