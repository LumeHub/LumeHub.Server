use serde::{Deserialize, de::DeserializeOwned};

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

impl CommandRequest {
    pub fn get_params<T: DeserializeOwned>(&self) -> Result<T, String> {
        let params_value = self
            .execution
            .first()
            .ok_or_else(|| "badRequest".to_string())?
            .params
            .clone();

        serde_json::from_value(params_value).map_err(|_| "badRequest".to_string())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionRequest {
    pub command: ExecuteCommandType,
    pub params: serde_json::Value,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum ExecuteCommandType {
    #[serde(rename = "action.devices.commands.OnOff")]
    OnOff,
    #[serde(rename = "action.devices.commands.BrightnessAbsolute")]
    BrightnessAbsolute,
    #[serde(rename = "action.devices.commands.ColorAbsolute")]
    ColorAbsolute,
    #[serde(rename = "action.devices.commands.BrightnessRelative")]
    BrightnessRelative,
}

