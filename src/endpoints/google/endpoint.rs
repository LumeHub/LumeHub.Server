use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, post, web};

use super::device::{GoogleDevice, LedStrip};
use super::request::{Intent, RequestInput, SmartHomeRequest};
use super::response::{
    ExecuteResponse, QueryResponse, ResponsePayload, SmartHomeResponse, SyncResponse,
};
use crate::effects::EffectQueue;
use crate::state::LumeState;

use super::commands;

#[post("/smarthome")]
async fn smarthome(
    req: web::Json<SmartHomeRequest>,
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
    command_dispatcher: web::Data<commands::CommandDispatcher>,
) -> impl Responder {
    let response_payload = match req.inputs.first() {
        Some(input) => match input.intent {
            Intent::Sync => handle_sync(),
            Intent::Query => handle_query(input, &lume_state),
            Intent::Execute => {
                handle_execute(input, &lume_state, &effect_queue, &command_dispatcher)
            }
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

fn handle_sync() -> ResponsePayload {
    let device = LedStrip;
    let sync_response = SyncResponse {
        agent_user_id: "123".to_string(), // TODO: change to actual user ID management, when implemented
        devices: vec![device.sync()],
    };
    ResponsePayload::Sync(sync_response)
}

fn handle_query(input: &RequestInput, lume_state: &web::Data<Mutex<LumeState>>) -> ResponsePayload {
    let device = LedStrip;
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
    lume_state: &web::Data<Mutex<LumeState>>,
    effect_queue: &web::Data<EffectQueue>,
    command_dispatcher: &web::Data<commands::CommandDispatcher>,
) -> ResponsePayload {
    let mut lume_service = crate::lume_service::LumeService::new(lume_state, effect_queue);

    let command_responses = if let Some(payload) = input.payload.as_ref() {
        if let Some(commands_vec) = payload.commands.as_ref() {
            let mut responses = Vec::new();
            for cmd_req in commands_vec.iter() {
                responses.extend(command_dispatcher.process_command(cmd_req, &mut lume_service));
            }
            responses
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    ResponsePayload::Execute(ExecuteResponse {
        commands: command_responses,
    })
}
