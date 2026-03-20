use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Dynamic, Engine, ImmutableString, Scope};

use domain::{ParamDef, ParamValue, Rgb};

use super::{make_script_engine, parse_color};
use crate::compositor::layer::LayerEffect;
use crate::compositor::live_param::LiveParam;
use crate::error::EffectError;

pub struct ScriptLayer {
    engine: Engine,
    ast: AST,
    frame: AtomicU64,
    params: Vec<(ImmutableString, Dynamic)>,
    primary_color: Arc<LiveParam<Rgb>>,
    strip_len: usize,
    zone_start: usize,
}

impl LayerEffect for ScriptLayer {
    fn render(&self, len: usize) -> Vec<Rgb> {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);

        let mut scope = Scope::new();
        scope.push("len", len as i64);
        scope.push("strip_len", self.strip_len as i64);
        scope.push("zone_start", self.zone_start as i64);
        scope.push("time", frame as f64);
        scope.push("frame", frame as i64);
        scope.push("pi", std::f64::consts::PI);

        for (name, val) in &self.params {
            scope.push_dynamic(name.as_str(), val.clone());
        }
        scope.push("primary", self.primary_color.get());

        let base_len = scope.len();
        (0..len)
            .map(|i| {
                scope.push("pixel", i as i64);
                scope.push("t", i as f64 / (len as f64 - 1.0).max(1.0));
                let color = self
                    .engine
                    .eval_ast_with_scope::<Dynamic>(&mut scope, &self.ast)
                    .map(parse_color)
                    .unwrap_or_else(|e| {
                        if i == 0 {
                            eprintln!("warning: script eval error: {e}");
                        }
                        Rgb::BLACK
                    });
                scope.rewind(base_len);
                color
            })
            .collect()
    }
}

pub fn build_layer(
    script: &str,
    param_defs: &[ParamDef],
    layer_params: &HashMap<String, ParamValue>,
    primary_color: Arc<LiveParam<Rgb>>,
    strip_len: usize,
    zone_start: usize,
) -> Result<Arc<dyn LayerEffect>, EffectError> {
    let params: Vec<(ImmutableString, Dynamic)> = param_defs
        .iter()
        .map(|def| {
            let value = layer_params.get(&def.name).unwrap_or(&def.default);
            (ImmutableString::from(&def.name), param_to_dynamic(value))
        })
        .collect();

    let engine = make_script_engine();
    let ast = engine.compile(script).map_err(EffectError::ScriptCompile)?;

    Ok(Arc::new(ScriptLayer {
        engine,
        ast,
        frame: AtomicU64::new(0),
        params,
        primary_color,
        strip_len,
        zone_start,
    }))
}

pub fn param_to_dynamic(value: &ParamValue) -> Dynamic {
    match value {
        ParamValue::Number(n) => Dynamic::from(*n as f64),
        ParamValue::Color(rgb) => Dynamic::from(*rgb),
        ParamValue::Bool(b) => Dynamic::from(*b),
        ParamValue::Select(s) => Dynamic::from(s.clone()),
    }
}
