pub mod brightness_absolute;
pub mod brightness_relative;
pub mod color_absolute;
pub mod on_off;

use crate::lume_service::LumeService;
use serde::de::DeserializeOwned;

use super::request::{CommandRequest, ExecuteCommandType};
use super::response::{CommandResponse, CommandStatus};
use crate::endpoints::google::device::device_states_from_lume_state;

pub trait GoogleCommand: Send + Sync {
    fn command_type(&self) -> ExecuteCommandType;
    fn handle(
        &self,
        cmd_req: &CommandRequest,
        lume_service: &mut LumeService,
    ) -> Result<(), String>;
}

pub trait GoogleCommandWithParams: Send + Sync {
    type Params: DeserializeOwned;
    fn command_type(&self) -> ExecuteCommandType;
    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String>;
}

impl<T> GoogleCommand for T
where
    T: GoogleCommandWithParams + Sized + 'static,
{
    fn command_type(&self) -> ExecuteCommandType {
        <Self as GoogleCommandWithParams>::command_type(self)
    }

    fn handle(
        &self,
        cmd_req: &CommandRequest,
        lume_service: &mut LumeService,
    ) -> Result<(), String> {
        cmd_req
            .get_params::<T::Params>()
            .and_then(|params| self.handle(params, lume_service))
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
            ],
        }
    }

    pub fn process_command(
        &self,
        cmd_req: &CommandRequest,
        lume_service: &mut LumeService,
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
            match command_handler.handle(cmd_req, lume_service) {
                Ok(_) => cmd_req
                    .devices
                    .iter()
                    .map(|d| CommandResponse {
                        ids: vec![d.id.clone()],
                        status: CommandStatus::Success,
                        states: Some(device_states_from_lume_state(
                            &lume_service.lume_state.lock().unwrap(),
                        )),
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
