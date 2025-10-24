use super::response::{Device, DeviceInfo, Name};
use crate::{
    endpoints::google::{commands::color_absolute::ColorState, response::DeviceStates},
    state::LumeState,
};

const DEVICE_ID: &str = "led-strip";

pub trait GoogleDevice {
    fn sync(&self) -> Device;
    fn query(&self, state: &LumeState) -> serde_json::Value;
}

pub struct LedStrip;

impl GoogleDevice for LedStrip {
    fn sync(&self) -> Device {
        Device {
            id: DEVICE_ID.to_string(),
            device_type: "action.devices.types.LIGHT".to_string(),
            traits: vec![
                "action.devices.traits.OnOff".to_string(),
                "action.devices.traits.ColorSetting".to_string(),
                "action.devices.traits.Brightness".to_string(),
            ],
            name: Name {
                name: "LED Strip".to_string(),
            },
            will_report_state: false,
            device_info: DeviceInfo {
                manufacturer: "LumeHub".to_string(),
                model: "LEDv1".to_string(),
            },
            attributes: Some(serde_json::json!({
                "colorModel": "rgb",
                "colorTemperatureRange": {
                    "temperatureMinK": 2000,
                    "temperatureMaxK": 9000
                },
                "commandOnlyBrightness": false
            })),
        }
    }

    fn query(&self, state: &LumeState) -> serde_json::Value {
        serde_json::to_value(device_states_from_lume_state(state)).unwrap()
    }
}

pub fn device_states_from_lume_state(state: &LumeState) -> DeviceStates {
    DeviceStates {
        on: Some(state.is_on),
        online: Some(true),
        brightness: Some(state.brightness),
        color: Some(ColorState {
            spectrum_rgb: Some(state.active_color.to_spectrum()),
            spectrum_hsv: None,
            temperature_k: None,
        }),
    }
}
