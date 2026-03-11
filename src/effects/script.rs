use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Engine, Scope};
use serde::Deserialize;
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::builtins::make_script_engine;
use crate::effects::composite::live_param::{PRIMARY_COLOR, SECONDARY_COLOR};
use crate::effects::composite::{LayerEffect, ParameterBus};
use crate::effects::registry::{EffectBuildError, EffectRegistry};
use crate::effects::rhai::parse_color;

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

        let mut pr = 1.0_f64;
        let mut pg = 1.0_f64;
        let mut pb = 1.0_f64;
        let mut sr = 0.0_f64;
        let mut sg = 0.0_f64;
        let mut sb = 0.0_f64;
        for (name, param) in self.bus.all_colors() {
            let c = param.get();
            scope.push(name.as_str(), c);
            let (rf, gf, bf) = (c.r as f64 / 255.0, c.g as f64 / 255.0, c.b as f64 / 255.0);
            if name == PRIMARY_COLOR {
                pr = rf;
                pg = gf;
                pb = bf;
            }
            if name == SECONDARY_COLOR {
                sr = rf;
                sg = gf;
                sb = bf;
            }
        }
        scope.push("pr", pr);
        scope.push("pg", pg);
        scope.push("pb", pb);
        scope.push("sr", sr);
        scope.push("sg", sg);
        scope.push("sb", sb);

        for (name, val) in &self.extra_vars {
            scope.push(name.as_str(), *val);
        }

        let base_len = scope.len();
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            scope.push("pixel", i as i64);
            scope.push("t", i as f64 / (len as f64 - 1.0).max(1.0));
            let color = match self
                .engine
                .eval_ast_with_scope::<rhai::Dynamic>(&mut scope, &self.ast)
            {
                Ok(val) => parse_color(val),
                Err(_) => Rgb::BLACK,
            };
            scope.rewind(base_len);
            result.push(color);
        }
        result
    }
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register_layer(
        "script",
        |params: Value, bus: Arc<ParameterBus>, prelude: &str| {
            let p: ScriptParams =
                serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;

            let mut extra_vars: Vec<(String, f64)> = p
                .vars
                .into_iter()
                .filter_map(|(k, v)| v.as_f64().map(|f| (k, f)))
                .collect();
            if !extra_vars.iter().any(|(k, _)| k == "speed") {
                extra_vars.push(("speed".to_string(), 1.0));
            }

            let engine = make_script_engine();
            let full_code = format!("{prelude}\n{code}", prelude = prelude, code = p.code);
            let ast = engine
                .compile(&full_code)
                .map_err(|e| EffectBuildError(format!("script compile error: {}", e)))?;

            Ok(Arc::new(Script {
                engine,
                ast,
                bus,
                extra_vars,
                frame: AtomicU64::new(0),
            }) as Arc<dyn LayerEffect>)
        },
    );
}
