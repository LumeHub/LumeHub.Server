use serde::Deserialize;
use serde::Serialize;

use crate::color::Rgb;
use crate::lume_service::LumeService;
use hsv::hsv_to_rgb;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;

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

fn parse_color_params(color_params: &ColorAbsoluteColor) -> Result<Rgb, String> {
    if let Some(spectrum_rgb) = color_params.spectrum_rgb {
        Ok(parse_spectrum_rgb(spectrum_rgb).0)
    } else if let Some(spectrum_hsv) = &color_params.spectrum_hsv {
        Ok(parse_spectrum_hsv(spectrum_hsv).0)
    } else if let Some(kelvin) = color_params.temperature {
        Ok(parse_temperature_k(kelvin).0)
    } else {
        Err("noColorProvided".to_string())
    }
}

pub struct ColorAbsoluteCommand;

impl GoogleCommandWithParams for ColorAbsoluteCommand {
    type Params = ColorAbsoluteParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::ColorAbsolute
    }

    fn handle(&self, params: Self::Params, lume_service: &mut LumeService) -> Result<(), String> {
        let rgb_color = parse_color_params(&params.color)?;
        lume_service.set_color(rgb_color);
        Ok(())
    }
}
