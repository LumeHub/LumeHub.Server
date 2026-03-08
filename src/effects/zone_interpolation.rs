use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::Effect;
use crate::effects::composite::ParameterBus;
use crate::effects::composite::{ColorSource, LayerEffect};
use crate::effects::registry::{EffectBuildError, EffectRegistry};
use crate::settings::BindingConfig;

pub struct ZoneInterpolation {
    pub color1: Rgb,
    pub color2: Rgb,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

#[derive(Deserialize)]
pub struct ZoneInterpolationParams {
    pub color1: Option<Rgb>,
    pub color2: Option<Rgb>,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register("zone_interpolation", |params: Value| {
        let p: ZoneInterpolationParams =
            serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;
        Ok(Box::new(ZoneInterpolation {
            color1: p.color1.unwrap_or(Rgb::BLACK),
            color2: p.color2.unwrap_or(Rgb::BLACK),
            interpolation_length: p.interpolation_length,
            midpoint: p.midpoint,
        }))
    });

    registry.register_layer(
        "zone_interpolation",
        |params: Value, bindings: &HashMap<String, BindingConfig>, bus: &ParameterBus| {
            let p: ZoneInterpolationParams =
                serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;

            let color1 =
                build_color_source("color1", p.color1.unwrap_or(Rgb::BLACK), bindings, bus)?;
            let color2 =
                build_color_source("color2", p.color2.unwrap_or(Rgb::BLACK), bindings, bus)?;

            Ok(Arc::new(ZoneInterpolationLayer {
                color1,
                color2,
                interpolation_length: p.interpolation_length,
                midpoint: p.midpoint,
            }) as Arc<dyn LayerEffect>)
        },
    );
}

fn build_color_source(
    field: &str,
    static_val: Rgb,
    bindings: &HashMap<String, BindingConfig>,
    bus: &ParameterBus,
) -> Result<ColorSource, EffectBuildError> {
    if let Some(binding) = bindings.get(field) {
        let param = bus.get_color(&binding.signal).ok_or_else(|| {
            EffectBuildError(format!(
                "signal '{}' not found on parameter bus",
                binding.signal
            ))
        })?;
        Ok(ColorSource::Live(param))
    } else {
        Ok(ColorSource::Static(static_val))
    }
}

impl Effect for ZoneInterpolation {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let frame = render_zone(
            pixels.len(),
            self.color1,
            self.color2,
            self.interpolation_length,
            self.midpoint,
        );
        Box::new(std::iter::once(frame))
    }
}

pub struct ZoneInterpolationLayer {
    pub color1: ColorSource,
    pub color2: ColorSource,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

impl LayerEffect for ZoneInterpolationLayer {
    fn render(&self, len: usize) -> Vec<Rgb> {
        render_zone(
            len,
            self.color1.get(),
            self.color2.get(),
            self.interpolation_length,
            self.midpoint,
        )
    }
}

fn render_zone(
    len: usize,
    color1: Rgb,
    color2: Rgb,
    interpolation_length: usize,
    midpoint: usize,
) -> Vec<Rgb> {
    if interpolation_length == 0 {
        return (0..len)
            .map(|i| if i < midpoint { color1 } else { color2 })
            .collect();
    }

    let interp_start = midpoint.saturating_sub(interpolation_length / 2);
    let interp_end = midpoint + interpolation_length / 2;

    (0..len)
        .map(|i| {
            if i < interp_start {
                color1
            } else if i >= interp_end {
                color2
            } else {
                let t = (i - interp_start) as f32 / interpolation_length as f32;
                color1.lerp(color2, t)
            }
        })
        .collect()
}
