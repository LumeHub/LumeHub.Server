use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use super::bus::{DEFAULT_SIGNAL_SPEED, PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};
use super::composite::CompositeEffect;
use super::layer::CompositeLayer;
use super::registry::EffectRegistry;
use crate::preset::{EffectPreset, LayerPreset, SignalDef};
use domain::{BlendMode, Rgb};

pub struct InitialState {
    pub color: Rgb,
    pub brightness: u8,
    pub brightness_speed: f32,
    pub signal_colors: HashMap<String, Rgb>,
}

struct SignalInit {
    color: Rgb,
    speed: f32,
}

pub fn build_prelude(functions: &HashMap<String, String>) -> String {
    functions.values().cloned().collect::<Vec<_>>().join("\n")
}

pub fn build_composite(
    registry: &EffectRegistry,
    preset: &EffectPreset,
    state: &InitialState,
    prelude: String,
    all_presets: &HashMap<String, EffectPreset>,
) -> Result<(CompositeEffect, Arc<ParameterBus>), String> {
    let all_signals = collect_all_signals(preset, all_presets, 0);

    let mut signal_defs: HashMap<String, SignalInit> = state
        .signal_colors
        .iter()
        .map(|(name, &color)| {
            (
                name.clone(),
                SignalInit {
                    color,
                    speed: DEFAULT_SIGNAL_SPEED,
                },
            )
        })
        .chain([
            (
                PRIMARY_COLOR.to_string(),
                SignalInit {
                    color: state.color,
                    speed: DEFAULT_SIGNAL_SPEED,
                },
            ),
            (
                SECONDARY_COLOR.to_string(),
                SignalInit {
                    color: state
                        .signal_colors
                        .get(SECONDARY_COLOR)
                        .copied()
                        .unwrap_or(Rgb::BLACK),
                    speed: DEFAULT_SIGNAL_SPEED,
                },
            ),
        ])
        .collect();
    signal_defs.extend(
        all_signals
            .iter()
            .map(|(name, def)| (name.clone(), resolve_signal_init(name, def, state))),
    );

    let bus = Arc::new(signal_defs.into_iter().fold(
        ParameterBus::new(state.brightness as f32, state.brightness_speed),
        |mut bus, (name, init)| {
            bus.register_color(name, init.color, init.speed);
            bus
        },
    ));

    all_signals
        .iter()
        .filter_map(|(name, def)| match def {
            SignalDef::Script(code) => Some((name, code)),
            _ => None,
        })
        .try_for_each(|(name, code)| {
            bus.set_animated(name, code)
                .map_err(|e| format!("preset signal '{}' script error: {}", name, e))
        })?;

    let layers = build_layers(registry, &preset.layers, &bus, &prelude, all_presets, 0)?;
    Ok((
        CompositeEffect {
            layers,
            bus: Arc::clone(&bus),
        },
        bus,
    ))
}

fn resolve_signal_init(name: &str, def: &SignalDef, state: &InitialState) -> SignalInit {
    match def {
        SignalDef::Color(c) if name == PRIMARY_COLOR => SignalInit {
            color: state.color,
            speed: c.speed.unwrap_or(DEFAULT_SIGNAL_SPEED),
        },
        SignalDef::Color(c) => SignalInit {
            color: state
                .signal_colors
                .get(name)
                .copied()
                .unwrap_or(Rgb::from(c)),
            speed: c.speed.unwrap_or(DEFAULT_SIGNAL_SPEED),
        },
        SignalDef::Script(_) => SignalInit {
            color: state.signal_colors.get(name).copied().unwrap_or(Rgb::BLACK),
            speed: DEFAULT_SIGNAL_SPEED,
        },
    }
}

fn collect_all_signals(
    preset: &EffectPreset,
    all_presets: &HashMap<String, EffectPreset>,
    depth: usize,
) -> HashMap<String, SignalDef> {
    if depth > 8 {
        return HashMap::new();
    }
    let mut signals = preset
        .layers
        .iter()
        .filter_map(|layer| all_presets.get(&layer.effect))
        .flat_map(|sub| collect_all_signals(sub, all_presets, depth + 1))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert(v);
            acc
        });
    signals.extend(preset.signals.iter().map(|(k, v)| (k.clone(), v.clone())));
    signals
}

fn build_layers(
    registry: &EffectRegistry,
    layers: &[LayerPreset],
    bus: &Arc<ParameterBus>,
    prelude: &str,
    all_presets: &HashMap<String, EffectPreset>,
    depth: usize,
) -> Result<Vec<CompositeLayer>, String> {
    if depth > 8 {
        return Err("preset nesting too deep (max 8)".into());
    }
    layers.iter().try_fold(Vec::new(), |mut acc, layer| {
        if let Some(sub_preset) = all_presets.get(&layer.effect) {
            let merged = merge_sub_layers(&sub_preset.layers, &layer.params);
            let sub = build_layers(registry, &merged, bus, prelude, all_presets, depth + 1)?;
            acc.extend(with_parent_overrides(sub, layer));
        } else {
            acc.push(leaf_layer(registry, layer, bus, prelude)?);
        }
        Ok(acc)
    })
}

fn merge_sub_layers(
    sub: &[LayerPreset],
    parent_params: &HashMap<String, Value>,
) -> Vec<LayerPreset> {
    sub.iter()
        .map(|sl| LayerPreset {
            params: sl
                .params
                .iter()
                .chain(parent_params.iter())
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            ..sl.clone()
        })
        .collect()
}

fn with_parent_overrides(
    mut layers: Vec<CompositeLayer>,
    parent: &LayerPreset,
) -> Vec<CompositeLayer> {
    if let Some(ref mode_str) = parent.mode {
        let mode = BlendMode::from(mode_str.as_str());
        layers.iter_mut().for_each(|l| l.mode = mode);
    }
    if let Some(og) = parent.opacity_gradient {
        layers
            .iter_mut()
            .for_each(|l| l.opacity_gradient = Some(og));
    }
    layers
}

fn leaf_layer(
    registry: &EffectRegistry,
    layer: &LayerPreset,
    bus: &Arc<ParameterBus>,
    prelude: &str,
) -> Result<CompositeLayer, String> {
    let params = Value::Object(layer.params.clone().into_iter().collect());
    registry
        .build_layer(&layer.effect, params, Arc::clone(bus), prelude)
        .map(|effect| CompositeLayer {
            effect,
            mode: layer
                .mode
                .as_deref()
                .map(BlendMode::from)
                .unwrap_or(BlendMode::Override),
            opacity_gradient: layer.opacity_gradient,
        })
        .map_err(|e| format!("error building layer '{}': {}", layer.effect, e.0))
}
