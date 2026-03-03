use serde::Deserialize;
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::Effect;
use crate::effects::registry::{EffectBuildError, EffectRegistry};

pub struct ZoneInterpolation {
    pub color1: Rgb,
    pub color2: Rgb,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

#[derive(Deserialize)]
pub struct ZoneInterpolationParams {
    pub color1: Rgb,
    pub color2: Rgb,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register("zone_interpolation", |params: Value| {
        let p: ZoneInterpolationParams =
            serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;
        Ok(Box::new(ZoneInterpolation {
            color1: p.color1,
            color2: p.color2,
            interpolation_length: p.interpolation_length,
            midpoint: p.midpoint,
        }))
    });
}

impl Effect for ZoneInterpolation {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let mut frame = vec![Rgb::BLACK; len];

        if self.interpolation_length == 0 {
            (0..len).for_each(|i| {
                if i < self.midpoint {
                    frame[i] = self.color1;
                } else {
                    frame[i] = self.color2;
                }
            });
            return Box::new(std::iter::once(frame));
        }

        let interp_start = self.midpoint.saturating_sub(self.interpolation_length / 2);
        let interp_end = self.midpoint + self.interpolation_length / 2;

        (0..len).for_each(|i| {
            if i < interp_start {
                frame[i] = self.color1;
            } else if i >= interp_end {
                frame[i] = self.color2;
            } else {
                let t = (i - interp_start) as f32 / self.interpolation_length as f32;
                frame[i] = self.color1.lerp(self.color2, t);
            }
        });

        Box::new(std::iter::once(frame))
    }
}
