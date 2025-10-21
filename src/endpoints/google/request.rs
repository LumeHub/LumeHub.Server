use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartHomeRequest {
    pub request_id: String,
    pub inputs: Vec<RequestInput>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInput {
    pub intent: Intent,
    pub payload: Option<RequestPayload>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Intent {
    #[serde(rename = "action.devices.SYNC")]
    Sync,
    #[serde(rename = "action.devices.QUERY")]
    Query,
    #[serde(rename = "action.devices.EXECUTE")]
    Execute,
    #[serde(rename = "action.devices.DISCONNECT")]
    Disconnect,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPayload {
    pub devices: Option<Vec<DeviceRequest>>,
    pub commands: Option<Vec<CommandRequest>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    pub devices: Vec<DeviceRequest>,
    pub execution: Vec<ExecutionRequest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionRequest {
    pub command: String,
    pub params: serde_json::Value,
}
