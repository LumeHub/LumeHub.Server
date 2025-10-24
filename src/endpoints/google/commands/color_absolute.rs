use std::sync::MutexGuard;

use serde::Deserialize;
use serde::Serialize;

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;
use hsv::hsv_to_rgb;

use super::super::request::CommandRequest;
use super::super::response::{CommandResponse, CommandStatus, DeviceStates};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ColorAbsoluteColor {
    pub name: Option<String>,
    pub temperature: Option<u32>,
    #[serde(rename = "spectrumRGB")]
    pub spectrum_rgb: Option<u32>,
    #[serde(rename = "spectrumHSV")]
    pub spectrum_hsv: Option<ColorAbsoluteHsv>,
}

#[derive(Debug, Deserialize)]
pub struct ColorAbsoluteHsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorAbsoluteParams {
    pub color: ColorAbsoluteColor,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spectrumRGB")]
    pub spectrum_rgb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spectrumHSV")]
    pub spectrum_hsv: Option<ColorStateHsv>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ColorStateHsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

fn parse_spectrum_rgb(spectrum_rgb: u32) -> (Rgb, ColorState) {
    let color = Rgb::from_spectrum_rgb(spectrum_rgb);
    (
        color,
        ColorState {
            spectrum_rgb: Some(spectrum_rgb),
            spectrum_hsv: None,
            temperature_k: None,
        },
    )
}

fn parse_spectrum_hsv(spectrum_hsv: &ColorAbsoluteHsv) -> (Rgb, ColorState) {
    let (r, g, b) = hsv_to_rgb(
        spectrum_hsv.hue as f64,
        spectrum_hsv.saturation as f64,
        spectrum_hsv.value as f64,
    );
    let color = Rgb { r, g, b };
    (
        color,
        ColorState {
            spectrum_rgb: None,
            spectrum_hsv: Some(ColorStateHsv {
                hue: spectrum_hsv.hue,
                saturation: spectrum_hsv.saturation,
                value: spectrum_hsv.value,
            }),
            temperature_k: None,
        },
    )
}

fn parse_temperature_k(kelvin: u32) -> (Rgb, ColorState) {
    let color = Rgb::from_temperature_k(kelvin);
    (
        color,
        ColorState {
            spectrum_rgb: None,
            spectrum_hsv: None,
            temperature_k: Some(kelvin),
        },
    )
}

fn parse_color_params(
    color_params: &ColorAbsoluteColor,
    make_error_responses: impl Fn(&str) -> Vec<CommandResponse>,
) -> Result<(Rgb, ColorState), Vec<CommandResponse>> {
    if let Some(spectrum_rgb) = color_params.spectrum_rgb {
        Ok(parse_spectrum_rgb(spectrum_rgb))
    } else if let Some(spectrum_hsv) = &color_params.spectrum_hsv {
        Ok(parse_spectrum_hsv(spectrum_hsv))
    } else if let Some(kelvin) = color_params.temperature {
        Ok(parse_temperature_k(kelvin))
    } else {
        Err(make_error_responses("noColorProvided"))
    }
}

pub fn handle_color_absolute_command(
    cmd_req: &CommandRequest,
    state: &mut MutexGuard<LumeState>,
    effect_queue: &EffectQueue,
    make_error_responses: impl Fn(&str) -> Vec<CommandResponse>,
) -> Vec<CommandResponse> {
    serde_json::from_value::<ColorAbsoluteParams>(cmd_req.execution.first().unwrap().params.clone())
        .map(|params| {
            let (rgb_color, color_state) =
                match parse_color_params(&params.color, &make_error_responses) {
                    Ok(val) => val,
                    Err(err_resp) => return err_resp,
                };

            state.active_color = rgb_color;
            state.is_on = true; // Setting color implies turning on

            effect_queue.enqueue(Box::new(FadeColor { color: rgb_color }));

            cmd_req
                .devices
                .iter()
                .map(|d| CommandResponse {
                    ids: vec![d.id.clone()],
                    status: CommandStatus::Success,
                    states: Some(DeviceStates {
                        on: Some(true),
                        online: Some(true),
                        brightness: Some(state.brightness),
                        color: Some(color_state.clone()),
                    }),
                    error_code: None,
                })
                .collect()
        })
        .unwrap_or_else(|_| make_error_responses("badRequest"))
}
