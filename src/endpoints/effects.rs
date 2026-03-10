use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::composite::{CompositeEffect, CompositeLayer, CompositeMode};
use crate::effects::composite::{PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};
use crate::effects::registry::EffectRegistry;
use crate::settings::{EffectPreset, EffectsConfig, LayerPreset, SignalDef};
use crate::state::LumeState;

fn build_prelude(functions: &HashMap<String, String>) -> String {
    functions.values().cloned().collect::<Vec<_>>().join("\n")
}

#[derive(Serialize)]
struct PresetsResponse<'a> {
    presets: &'a HashMap<String, EffectPreset>,
}

#[post("/effects/presets/{name}/execute")]
pub async fn execute_preset(
    effect_queue: web::Data<EffectQueue>,
    registry: web::Data<EffectRegistry>,
    lume_state: web::Data<Mutex<LumeState>>,
    effects: web::Data<EffectsConfig>,
    path: web::Path<String>,
) -> impl Responder {
    let preset_name = path.into_inner();
    let preset = match effects.presets.get(&preset_name) {
        Some(preset) => preset.clone(),
        None => return HttpResponse::NotFound().body("unknown preset"),
    };

    let (initial_color, signal_colors, signal_scripts, initial_brightness) = {
        let state = lume_state.lock().unwrap();
        (
            state.active_color,
            state.signal_colors.clone(),
            state.signal_scripts.clone(),
            state.brightness,
        )
    };

    let prelude = build_prelude(&effects.functions);
    match build_composite(
        &registry,
        &preset,
        initial_color,
        &signal_colors,
        initial_brightness,
        prelude,
        &effects.presets,
    ) {
        Ok((composite, bus)) => {
            // Apply user animated overrides — these beat preset animations.
            for (name, code) in &signal_scripts {
                let _ = bus.set_animated(name, code);
            }
            // Apply user static overrides — these beat everything (incl. animations).
            for (name, &color) in &signal_colors {
                bus.set_color(name, color);
            }

            let mut state = lume_state.lock().unwrap();
            state.is_on = true;
            state.active_light_effect = Some(preset_name);
            state.light_effect_end_unix_timestamp_sec = None;
            state.active_param_bus = Some(bus);
            effect_queue.enqueue(Box::new(composite));
            HttpResponse::Ok().finish()
        }
        Err(response) => response,
    }
}

/// Recursively collect all signals declared by a preset and any presets it
/// references as layers.  The outermost preset's signals take priority.
fn collect_all_signals(
    preset: &EffectPreset,
    all_presets: &HashMap<String, EffectPreset>,
    depth: usize,
) -> HashMap<String, SignalDef> {
    if depth > 8 {
        return HashMap::new();
    }
    let mut signals: HashMap<String, SignalDef> = HashMap::new();
    // Sub-preset signals first (lower priority — parent overrides them).
    for layer in &preset.layers {
        if let Some(sub) = all_presets.get(&layer.effect) {
            for (k, v) in collect_all_signals(sub, all_presets, depth + 1) {
                signals.entry(k).or_insert(v);
            }
        }
    }
    // This preset's own signals win.
    for (k, v) in &preset.signals {
        signals.insert(k.clone(), v.clone());
    }
    signals
}

/// Recursively build layers, expanding preset-name references inline.
/// When a layer's `effect` matches a known preset, that preset's layers are
/// substituted in place.  The referencing layer's `mode`/`opacity_gradient`
/// (if set) override those of every expanded sub-layer.
fn build_layers(
    registry: &EffectRegistry,
    layers: &[LayerPreset],
    bus: &Arc<ParameterBus>,
    all_presets: &HashMap<String, EffectPreset>,
    depth: usize,
) -> Result<Vec<CompositeLayer>, HttpResponse> {
    if depth > 8 {
        return Err(HttpResponse::BadRequest().body("preset nesting too deep (max 8)"));
    }
    let mut result = Vec::new();
    for layer in layers {
        if let Some(sub_preset) = all_presets.get(&layer.effect) {
            let mut sub_layers =
                build_layers(registry, &sub_preset.layers, bus, all_presets, depth + 1)?;
            if let Some(ref mode_str) = layer.mode {
                let mode = CompositeMode::from_str(mode_str);
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
                .build_layer(&layer.effect, params, Arc::clone(bus))
                .map_err(|err| {
                    HttpResponse::BadRequest().body(format!(
                        "Error building layer '{}': {}",
                        layer.effect, err.0
                    ))
                })?;
            result.push(CompositeLayer {
                effect,
                mode: layer
                    .mode
                    .as_deref()
                    .map(CompositeMode::from_str)
                    .unwrap_or(CompositeMode::Override),
                opacity_gradient: layer.opacity_gradient,
            });
        }
    }
    Ok(result)
}

fn build_composite(
    registry: &EffectRegistry,
    preset: &EffectPreset,
    initial_color: Rgb,
    signal_colors: &HashMap<String, Rgb>,
    initial_brightness: u8,
    prelude: String,
    all_presets: &HashMap<String, EffectPreset>,
) -> Result<(CompositeEffect, Arc<ParameterBus>), HttpResponse> {
    // Collect signals from this preset and all transitively referenced presets.
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
            SignalDef::Script(_) => {
                let color = signal_colors.get(name).copied().unwrap_or(Rgb::BLACK);
                (color, 5.0)
            }
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
        ParameterBus::new(initial_brightness as f32, prelude),
        |mut bus, (name, (initial, speed))| {
            bus.register_color(name, initial, speed);
            bus
        },
    );
    let bus = Arc::new(bus);

    // Apply animated signals from this preset and all referenced sub-presets.
    for (name, def) in &all_signals {
        if let SignalDef::Script(code) = def {
            if let Err(e) = bus.set_animated(name, code) {
                return Err(HttpResponse::BadRequest()
                    .body(format!("Preset signal '{}' script error: {}", name, e)));
            }
        }
    }

    let layers = build_layers(registry, &preset.layers, &bus, all_presets, 0)?;

    Ok((
        CompositeEffect {
            layers,
            bus: Arc::clone(&bus),
        },
        bus,
    ))
}

#[get("/effects/presets")]
pub async fn list_presets(effects: web::Data<EffectsConfig>) -> impl Responder {
    HttpResponse::Ok().json(PresetsResponse {
        presets: &effects.presets,
    })
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SetSignalRequest {
    Color { r: u8, g: u8, b: u8 },
    Script { code: String },
}

#[post("/effects/signals/{name}")]
pub async fn set_signal(
    lume_state: web::Data<Mutex<LumeState>>,
    path: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let req: SetSignalRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let signal_name = path.into_inner();
    let mut state = lume_state.lock().unwrap();

    match req {
        SetSignalRequest::Color { r, g, b } => {
            let color = Rgb { r, g, b };
            state.signal_scripts.remove(&signal_name);
            if signal_name == PRIMARY_COLOR {
                state.active_color = color;
            } else {
                state.signal_colors.insert(signal_name.clone(), color);
            }
            if let Some(bus) = &state.active_param_bus {
                bus.set_color(&signal_name, color);
            }
        }
        SetSignalRequest::Script { code } => {
            if let Some(bus) = &state.active_param_bus {
                if let Err(e) = bus.set_animated(&signal_name, &code) {
                    return HttpResponse::BadRequest().body(format!("script error: {}", e));
                }
            }
            state.signal_scripts.insert(signal_name, code);
        }
    }

    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(execute_preset)
        .service(list_presets)
        .service(set_signal);
}
