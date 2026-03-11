use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spectrumRGB")]
    pub spectrum_rgb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectrum_hsv: Option<ColorStateHsv>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorStateHsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartHomeResponse {
    pub request_id: String,
    pub payload: ResponsePayload,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ResponsePayload {
    Sync(SyncResponse),
    Query(QueryResponse),
    Execute(ExecuteResponse),
    Error(ErrorResponse),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResponse {
    pub agent_user_id: String,
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub traits: Vec<String>,
    pub name: Name,
    pub will_report_state: bool,
    pub device_info: DeviceInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub manufacturer: String,
    pub model: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    pub devices: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub commands: Vec<CommandResponse>,
}

#[derive(Debug, Serialize, PartialEq)]
#[allow(dead_code)]
pub enum CommandStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "OFFLINE")]
    Offline,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    pub ids: Vec<String>,
    pub status: CommandStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<DeviceStates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_light_effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_effect_end_unix_timestamp_sec: Option<u64>,
}
