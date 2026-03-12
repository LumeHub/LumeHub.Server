use super::response::{ColorState, Device, DeviceInfo, DeviceStates, Name};
use application::SceneSnapshot;

const DEVICE_ID: &str = "led-strip";

pub trait GoogleDevice {
    fn sync(&self) -> Device;
    fn query(&self, snap: &SceneSnapshot) -> serde_json::Value;
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
                "action.devices.traits.LightEffects".to_string(),
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
                "commandOnlyBrightness": false,
                "supportedEffects": ["colorLoop", "sleep", "wake"],
                "defaultColorLoopDuration": 1800,
                "defaultSleepDuration": 1800,
                "defaultWakeDuration": 1800
            })),
        }
    }

    fn query(&self, snap: &SceneSnapshot) -> serde_json::Value {
        serde_json::to_value(device_states_from_snapshot(snap)).unwrap()
    }
}

pub fn google_to_internal_brightness(pct: u8) -> u8 {
    (pct as f32 * 255.0 / 100.0).round() as u8
}

pub fn internal_to_google_brightness(val: u8) -> u8 {
    (val as f32 * 100.0 / 255.0).round() as u8
}

pub fn device_states_from_snapshot(snap: &SceneSnapshot) -> DeviceStates {
    DeviceStates {
        on: Some(snap.on),
        online: Some(true),
        brightness: Some(internal_to_google_brightness(snap.brightness)),
        color: Some(ColorState {
            spectrum_rgb: Some(snap.color.to_spectrum()),
            spectrum_hsv: None,
            temperature_k: None,
        }),
        active_light_effect: snap.active_effect.clone(),
        light_effect_end_unix_timestamp_sec: snap.light_effect_end_unix_timestamp_sec,
    }
}
