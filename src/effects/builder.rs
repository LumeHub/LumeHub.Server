use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::effects::composite::{CompositeEffect, CompositeLayer};
use crate::effects::composite::{PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};
use crate::effects::config::{EffectPreset, LayerPreset, SignalDef};
use crate::effects::registry::EffectRegistry;
use domain::{BlendMode, Rgb};

pub fn build_prelude(functions: &HashMap<String, String>) -> String {
    functions.values().cloned().collect::<Vec<_>>().join("\n")
}

fn collect_all_signals(
    preset: &EffectPreset,
    all_presets: &HashMap<String, EffectPreset>,
    depth: usize,
) -> HashMap<String, SignalDef> {
    if depth > 8 {
        return HashMap::new();
    }
    let mut signals: HashMap<String, SignalDef> = HashMap::new();
    for layer in &preset.layers {
        if let Some(sub) = all_presets.get(&layer.effect) {
            for (k, v) in collect_all_signals(sub, all_presets, depth + 1) {
                signals.entry(k).or_insert(v);
            }
        }
    }
    for (k, v) in &preset.signals {
        signals.insert(k.clone(), v.clone());
    }
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
    let mut result = Vec::new();
    for layer in layers {
        if let Some(sub_preset) = all_presets.get(&layer.effect) {
            // Extra params on the referencing layer override those of the sub-layers,
            // so callers can tweak individual parameters without rewriting the preset.
            let overridden: Vec<LayerPreset> = sub_preset
                .layers
                .iter()
                .map(|sl| {
                    let mut params = sl.params.clone();
                    params.extend(layer.params.clone());
                    LayerPreset {
                        params,
                        ..sl.clone()
                    }
                })
                .collect();
            let mut sub_layers =
                build_layers(registry, &overridden, bus, prelude, all_presets, depth + 1)?;
            if let Some(ref mode_str) = layer.mode {
                let mode = BlendMode::from(mode_str.as_str());
                for sl in &mut sub_layers {
                    sl.mode = mode.clone();
                }
            }
            if let Some(og) = layer.opacity_gradient {
                for sl in &mut sub_layers {
                    sl.opacity_gradient = Some(og);
                }
            }
            result.extend(sub_layers);
        } else {
            let params = Value::Object(layer.params.clone().into_iter().collect());
            let effect = registry
                .build_layer(&layer.effect, params, Arc::clone(bus), prelude)
                .map_err(|e| format!("error building layer '{}': {}", layer.effect, e.0))?;
            result.push(CompositeLayer {
                effect,
                mode: layer
                    .mode
                    .as_deref()
                    .map(BlendMode::from)
                    .unwrap_or(BlendMode::Override),
                opacity_gradient: layer.opacity_gradient,
            });
        }
    }
    Ok(result)
}

pub fn build_composite(
    registry: &EffectRegistry,
    preset: &EffectPreset,
    initial_color: Rgb,
    signal_colors: &HashMap<String, Rgb>,
    initial_brightness: u8,
    brightness_speed: f32,
    prelude: String,
    all_presets: &HashMap<String, EffectPreset>,
) -> Result<(CompositeEffect, Arc<ParameterBus>), String> {
    let all_signals = collect_all_signals(preset, all_presets, 0);

    let mut signal_defs: HashMap<String, (Rgb, f32)> = HashMap::new();
    for (name, def) in &all_signals {
        let (initial, speed) = match def {
            SignalDef::Color(c) => {
                let color = if name == PRIMARY_COLOR {
                    initial_color
                } else {
                    signal_colors.get(name).copied().unwrap_or(Rgb {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                    })
                };
                (color, c.speed.unwrap_or(5.0))
            }
            SignalDef::Script(_) => (signal_colors.get(name).copied().unwrap_or(Rgb::BLACK), 5.0),
        };
        signal_defs.insert(name.clone(), (initial, speed));
    }

    signal_defs
        .entry(PRIMARY_COLOR.to_string())
        .or_insert((initial_color, 5.0));
    signal_defs.entry(SECONDARY_COLOR.to_string()).or_insert((
        signal_colors
            .get(SECONDARY_COLOR)
            .copied()
            .unwrap_or(Rgb::BLACK),
        5.0,
    ));
    for (name, &color) in signal_colors {
        signal_defs.entry(name.clone()).or_insert((color, 5.0));
    }

    let bus = signal_defs.into_iter().fold(
        ParameterBus::new(initial_brightness as f32, brightness_speed),
        |mut bus, (name, (initial, speed))| {
            bus.register_color(name, initial, speed);
            bus
        },
    );
    let bus = Arc::new(bus);

    for (name, def) in &all_signals {
        if let SignalDef::Script(code) = def
            && let Err(e) = bus.set_animated(name, code)
        {
            return Err(format!("preset signal '{}' script error: {}", name, e));
        }
    }

    let layers = build_layers(registry, &preset.layers, &bus, &prelude, all_presets, 0)?;

    Ok((
        CompositeEffect {
            layers,
            bus: Arc::clone(&bus),
        },
        bus,
    ))
}
