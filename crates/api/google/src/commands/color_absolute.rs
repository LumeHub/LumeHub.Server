use serde::Deserialize;

use application::SceneRuntime;
use domain::Rgb;
use hsv::hsv_to_rgb;

use super::super::request::ExecuteCommandType;
use super::GoogleCommandWithParams;
use crate::error::GoogleCommandError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ColorAbsoluteColor {
    pub name: Option<String>,
    pub temperature: Option<u32>,
    #[serde(rename = "spectrumRGB")]
    pub spectrum_rgb: Option<u32>,
    #[serde(rename = "spectrumHSV")]
    pub spectrum_hsv: Option<SpectrumHsv>,
}

#[derive(Debug, Deserialize)]
pub struct SpectrumHsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorAbsoluteParams {
    pub color: ColorAbsoluteColor,
}

fn parse_color(color: &ColorAbsoluteColor) -> Result<Rgb, GoogleCommandError> {
    if let Some(spectrum_rgb) = color.spectrum_rgb {
        return Ok(Rgb::from_spectrum_rgb(spectrum_rgb));
    }
    if let Some(hsv) = &color.spectrum_hsv {
        let (r, g, b) = hsv_to_rgb(hsv.hue as f64, hsv.saturation as f64, hsv.value as f64);
        return Ok(Rgb { r, g, b });
    }
    if let Some(kelvin) = color.temperature {
        return Ok(Rgb::from_temperature_k(kelvin));
    }
    Err(GoogleCommandError::NoColorProvided)
}

pub struct ColorAbsoluteCommand;

impl GoogleCommandWithParams for ColorAbsoluteCommand {
    type Params = ColorAbsoluteParams;

    fn command_type(&self) -> ExecuteCommandType {
        ExecuteCommandType::ColorAbsolute
    }

    fn handle(
        &self,
        params: Self::Params,
        runtime: &dyn SceneRuntime,
    ) -> Result<(), GoogleCommandError> {
        let color = parse_color(&params.color)?;
        runtime.set_color(color);
        Ok(())
    }
}
