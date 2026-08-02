use std::collections::HashMap;
use std::sync::Arc;

use domain::{BlendMode, ParamDef, ParamValue, Rgb};

use crate::compositor::composite::CompositeEffect;
use crate::compositor::layer::{CompositeLayer, ZoneGradient};
use crate::compositor::live_param::LiveParam;
use crate::error::EffectError;
use crate::rhai::script;

pub struct LayerSpec<'a> {
    pub script: &'a str,
    pub param_defs: &'a [ParamDef],
    pub params: &'a HashMap<String, ParamValue>,
    pub blend_mode: BlendMode,
    pub zone_start: usize,
    pub zone_end: usize,
    pub zone_transition: usize,
    pub opacity: Arc<LiveParam<f32>>,
}

pub fn build_composite(
    layers: &[LayerSpec<'_>],
    strip_len: usize,
    brightness: Arc<LiveParam<f32>>,
    primary_color: Arc<LiveParam<Rgb>>,
) -> Result<CompositeEffect, EffectError> {
    let composite_layers = layers
        .iter()
        .map(|spec| {
            let effect = script::build_layer(
                spec.script,
                spec.param_defs,
                spec.params,
                Arc::clone(&primary_color),
                strip_len,
                spec.zone_start,
            )
            .map_err(|e| EffectError::LayerBuild {
                effect: spec.script.chars().take(40).collect(),
                source: Box::new(e),
            })?;

            Ok(CompositeLayer {
                effect,
                mode: spec.blend_mode,
                zone: ZoneGradient {
                    start_pixel: spec.zone_start,
                    end_pixel: spec.zone_end,
                    transition_length: spec.zone_transition,
                },
                opacity: Arc::clone(&spec.opacity),
            })
        })
        .collect::<Result<Vec<_>, EffectError>>()?;

    Ok(CompositeEffect {
        layers: composite_layers,
        brightness,
        primary_color,
    })
}
