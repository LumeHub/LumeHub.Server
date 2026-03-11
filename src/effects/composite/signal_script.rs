use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Engine, Scope};

use super::live_param::LiveParam;
use crate::color::Rgb;
use crate::effects::builtins::make_signal_engine;
use crate::effects::rhai::parse_color;

/// A Rhai script that drives a single color signal. Evaluated once per frame.
/// Scope: `time` (f64), `frame` (i64), `pi`.
/// Must return a color via rgb(), rgb_f(), from_hue(), etc.
pub struct SignalScript {
    engine: Engine,
    ast: AST,
    pub live: LiveParam<Rgb>,
    frame: AtomicU64,
}

impl SignalScript {
    pub fn new(code: &str) -> Result<Arc<Self>, String> {
        let engine = make_signal_engine();
        let ast = engine.compile(code).map_err(|e| e.to_string())?;
        Ok(Arc::new(Self {
            engine,
            ast,
            live: LiveParam::new(Rgb::BLACK, f32::MAX),
            frame: AtomicU64::new(0),
        }))
    }

    pub fn tick(&self) {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);
        let mut scope = Scope::new();
        scope.push("time", frame as f64);
        scope.push("frame", frame as i64);
        scope.push("pi", std::f64::consts::PI);
        let color = match self
            .engine
            .eval_ast_with_scope::<rhai::Dynamic>(&mut scope, &self.ast)
        {
            Ok(val) => parse_color(val),
            Err(_) => Rgb::BLACK,
        };
        self.live.set_immediate(color);
    }
}
