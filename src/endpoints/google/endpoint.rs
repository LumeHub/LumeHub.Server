use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use actix_web::{HttpResponse, Responder, post, web};

use super::device::{GoogleDevice, LedStrip, OnOffParams};
use super::request::{CommandRequest, Intent, RequestInput, SmartHomeRequest};
use super::response::{
    CommandResponse, CommandStatus, ExecuteResponse, QueryResponse, ResponsePayload,
    SmartHomeResponse, SyncResponse,
};
use crate::effects::EffectQueue;
use crate::state::LumeState;

#[post("/smarthome")]
async fn smarthome(
    req: web::Json<SmartHomeRequest>,
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    let device = LedStrip;

    let response_payload = match req.inputs.first() {
        Some(input) => match input.intent {
            Intent::Sync => handle_sync(&device),
            Intent::Query => handle_query(input, &device, &lume_state),
            Intent::Execute => handle_execute(input, &device, &lume_state, &effect_queue),
            Intent::Disconnect => ResponsePayload::Error(super::response::ErrorResponse {
                error_code: "Unsupported intent: DISCONNECT".to_string(),
            }),
        },
        None => ResponsePayload::Error(super::response::ErrorResponse {
            error_code: "No input found in request".to_string(),
        }),
    };

    HttpResponse::Ok().json(SmartHomeResponse {
        request_id: req.request_id.clone(),
        payload: response_payload,
    })
}

fn handle_sync(device: &impl GoogleDevice) -> ResponsePayload {
    let sync_response = SyncResponse {
        agent_user_id: "123".to_string(), // TODO: change to actual user ID management, when implemented
        devices: vec![device.sync()],
    };
    ResponsePayload::Sync(sync_response)
}

fn handle_query(
    input: &RequestInput,
    device: &impl GoogleDevice,
    lume_state: &web::Data<Mutex<LumeState>>,
) -> ResponsePayload {
    let state = lume_state.lock().unwrap();

    let devices_response: HashMap<String, serde_json::Value> = input
        .payload
        .as_ref()
        .and_then(|p| p.devices.as_ref())
        .map_or_else(HashMap::new, |devices_to_query| {
            devices_to_query
                .iter()
                .filter(|d| d.id == device.sync().id)
                .map(|d| (d.id.clone(), device.query(&state)))
                .collect()
        });

    ResponsePayload::Query(QueryResponse {
        devices: devices_response,
    })
}

fn handle_execute(
    input: &RequestInput,
    device: &impl GoogleDevice,
    lume_state: &web::Data<Mutex<LumeState>>,
    effect_queue: &web::Data<EffectQueue>,
) -> ResponsePayload {
    let mut state = lume_state.lock().unwrap();

    let command_responses: Vec<CommandResponse> = input
        .payload
        .as_ref()
        .and_then(|p| p.commands.as_ref())
        .map_or_else(Vec::new, |commands| {
            commands
                .iter()
                .flat_map(|cmd_req| process_command(cmd_req, device, &mut state, effect_queue))
                .collect()
        });

    ResponsePayload::Execute(ExecuteResponse {
        commands: command_responses,
    })
}

fn process_command(
    cmd_req: &CommandRequest,
    device: &impl GoogleDevice,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
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

    match exec_req.command.as_str() {
        "action.devices.commands.OnOff" => {
            serde_json::from_value::<OnOffParams>(exec_req.params.clone())
                .map(|params| {
                    cmd_req
                        .devices
                        .iter()
                        .map(|_d| device.execute_on_off(&params, state, effect_queue))
                        .collect()
                })
                .unwrap_or_else(|_| make_error_responses("badRequest"))
        }
        _ => make_error_responses("unsupportedCommand"),
    }
}
