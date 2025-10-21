use std::collections::HashMap;
use std::sync::MutexGuard;

use serde::Deserialize;
use serde_json::json;

use super::response::{CommandResponse, CommandStatus, Device, DeviceInfo, Name};
use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;

const DEVICE_ID: &str = "led-strip";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnOffParams {
    pub on: bool,
}

pub trait GoogleDevice {
    fn sync(&self) -> Device;

    fn query(&self, state: &LumeState) -> serde_json::Value;

    fn execute_on_off(
        &self,
        params: &OnOffParams,
        state: &mut MutexGuard<LumeState>,
        effect_queue: &EffectQueue,
    ) -> CommandResponse;
}

pub struct LedStrip;

impl GoogleDevice for LedStrip {
    fn sync(&self) -> Device {
        Device {
            id: DEVICE_ID.to_string(),
            device_type: "action.devices.types.LIGHT".to_string(),
            traits: vec!["action.devices.traits.OnOff".to_string()],
            name: Name {
                name: "LED Strip".to_string(),
            },
            will_report_state: false,
            device_info: DeviceInfo {
                manufacturer: "LumeHub".to_string(),
                model: "LEDv1".to_string(),
            },
        }
    }

    fn query(&self, state: &LumeState) -> serde_json::Value {
        json!({ "on": state.is_on, "online": true })
    }

    fn execute_on_off(
        &self,
        params: &OnOffParams,
        state: &mut MutexGuard<LumeState>,
        effect_queue: &EffectQueue,
    ) -> CommandResponse {
        state.is_on = params.on;
        let effect = if params.on {
            Box::new(FadeColor {
                color: state.active_color,
            })
        } else {
            Box::new(FadeColor { color: Rgb::BLACK })
        };
        effect_queue.enqueue(effect);

        CommandResponse {
            ids: vec![DEVICE_ID.to_string()],
            status: CommandStatus::Success,
            states: Some(HashMap::from([("on".to_string(), json!(params.on))])),
            error_code: None,
        }
    }
}
