use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Engine, Scope};
use serde::Deserialize;
use serde_json::Value;

use super::{make_script_engine, parse_color};
use crate::compositor::bus::{PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};
use crate::compositor::layer::LayerEffect;
use crate::compositor::registry::EffectRegistry;
use crate::error::EffectError;
use domain::Rgb;

#[derive(Deserialize)]
struct ScriptParams {
    code: String,
    #[serde(flatten)]
    vars: HashMap<String, Value>,
}

pub struct Script {
    engine: Engine,
    ast: AST,
    bus: Arc<ParameterBus>,
    extra_vars: Vec<(String, f64)>,
    frame: AtomicU64,
}

impl LayerEffect for Script {
    fn render(&self, len: usize) -> Vec<Rgb> {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);

        let mut scope = Scope::new();
        scope.push("len", len as i64);
        scope.push("time", frame as f64);
        scope.push("frame", frame as i64);
        scope.push("pi", std::f64::consts::PI);

        let colors = self.bus.all_colors();
        for (name, color) in &colors {
            scope.push(name.clone(), *color);
        }

        let primary = colors
            .get(PRIMARY_COLOR)
            .copied()
            .unwrap_or(Rgb::new(255, 255, 255));
        let secondary = colors.get(SECONDARY_COLOR).copied().unwrap_or(Rgb::BLACK);
        let to_f = |c: u8| c as f64 / 255.0;
        scope.push("pr", to_f(primary.r));
        scope.push("pg", to_f(primary.g));
        scope.push("pb", to_f(primary.b));
        scope.push("sr", to_f(secondary.r));
        scope.push("sg", to_f(secondary.g));
        scope.push("sb", to_f(secondary.b));

        for (name, val) in &self.extra_vars {
            scope.push(name.as_str(), *val);
        }

        let base_len = scope.len();
        (0..len)
            .map(|i| {
                scope.push("pixel", i as i64);
                scope.push("t", i as f64 / (len as f64 - 1.0).max(1.0));
                let color = self
                    .engine
                    .eval_ast_with_scope::<rhai::Dynamic>(&mut scope, &self.ast)
                    .map(parse_color)
                    .unwrap_or(Rgb::BLACK);
                scope.rewind(base_len);
                color
            })
            .collect()
    }
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register_layer("script", |params, bus, prelude| {
        let p: ScriptParams =
            serde_json::from_value(params).map_err(|e| EffectError::InvalidParams {
                effect: "script".to_string(),
                source: e,
            })?;

        let mut extra_vars: Vec<(String, f64)> = p
            .vars
            .into_iter()
            .filter_map(|(k, v)| v.as_f64().map(|f| (k, f)))
            .collect();
        if !extra_vars.iter().any(|(k, _)| k == "speed") {
            extra_vars.push(("speed".to_string(), 1.0));
        }

        let engine = make_script_engine();
        let full_code = format!("{prelude}\n{code}", code = p.code);
        let ast = engine
            .compile(&full_code)
            .map_err(EffectError::ScriptCompile)?;

        Ok(Arc::new(Script {
            engine,
            ast,
            bus,
            extra_vars,
            frame: AtomicU64::new(0),
        }) as Arc<dyn LayerEffect>)
    });
}
