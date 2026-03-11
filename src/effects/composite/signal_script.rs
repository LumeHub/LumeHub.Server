use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Engine, Scope};

use super::live_param::LiveParam;
use crate::effects::builtins::make_signal_engine;
use crate::effects::rhai::parse_color;
use domain::Rgb;

/// A Rhai script that drives a single color signal. Evaluated once per frame.
/// Scope: `time` (f64), `frame` (i64), `pi`.
/// Must return a color via rgb(), rgb_f(), from_hue(), etc.
pub struct SignalScript {
    engine: Engine,
    ast: AST,
    pub live: LiveParam<Rgb>,
    frame: AtomicU64,
    from_color: Rgb,
    fade_frames: usize,
}

impl SignalScript {
    pub fn new(code: &str, from_color: Rgb, fade_frames: usize) -> Result<Arc<Self>, String> {
        let engine = make_signal_engine();
        let ast = engine.compile(code).map_err(|e| e.to_string())?;
        Ok(Arc::new(Self {
            engine,
            ast,
            live: LiveParam::new(from_color, f32::MAX),
            frame: AtomicU64::new(0),
            from_color,
            fade_frames,
        }))
    }

    pub fn tick(&self) {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);
        let mut scope = Scope::new();
        scope.push("time", frame as f64);
        scope.push("frame", frame as i64);
        scope.push("pi", std::f64::consts::PI);
        let script_color = match self
            .engine
            .eval_ast_with_scope::<rhai::Dynamic>(&mut scope, &self.ast)
        {
            Ok(val) => parse_color(val),
            Err(_) => Rgb::BLACK,
        };
        let color = if self.fade_frames > 0 && (frame as usize) < self.fade_frames {
            let t = (frame as f32 + 1.0) / self.fade_frames as f32;
            self.from_color.lerp(script_color, t)
        } else {
            script_color
        };
        self.live.set_immediate(color);
    }
}
