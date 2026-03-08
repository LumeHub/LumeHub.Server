use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Deserialize;
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::composite::{LayerEffect, ParameterBus};
use crate::effects::registry::{EffectBuildError, EffectRegistry};
use crate::settings::BindingConfig;

#[derive(Deserialize)]
struct RainbowZoneParams {
    start: usize,
    end: usize,
    speed: Option<f32>,
}

pub struct RainbowZone {
    start: usize,
    end: usize,
    speed: f32,
    frame: AtomicU64,
}

impl LayerEffect for RainbowZone {
    fn render(&self, len: usize) -> Vec<Rgb> {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);
        let hue_offset = (frame as f32 * self.speed).rem_euclid(360.0);
        let end = self.end.min(len.saturating_sub(1));

        if self.start > end {
            return vec![Rgb::BLACK; len];
        }

        let zone_len = (end - self.start) as f32;
        let start = self.start;

        (0..len)
            .map(|i| {
                if i < start || i > end {
                    return Rgb::BLACK;
                }
                let t = (i - start) as f32 / zone_len;
                Rgb::from_hue(t * 360.0 + hue_offset)
            })
            .collect()
    }
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register_layer(
        "rainbow_zone",
        |params: Value, _bindings: &HashMap<String, BindingConfig>, _bus: &ParameterBus| {
            let p: RainbowZoneParams =
                serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;
            Ok(Arc::new(RainbowZone {
                start: p.start,
                end: p.end,
                speed: p.speed.unwrap_or(1.0),
                frame: AtomicU64::new(0),
            }) as Arc<dyn LayerEffect>)
        },
    );
}
