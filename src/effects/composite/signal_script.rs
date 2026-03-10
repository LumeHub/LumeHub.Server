use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Dynamic, Engine, Scope};

use super::live_param::LiveParam;
use crate::color::Rgb;
use crate::effects::rhai::{map_get, parse_color, rgb_map};

fn make_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(1_000);

    engine.register_fn("sin", |x: f64| -> f64 { x.sin() });
    engine.register_fn("cos", |x: f64| -> f64 { x.cos() });
    engine.register_fn("sqrt", |x: f64| -> f64 { x.sqrt() });
    engine.register_fn("abs", |x: f64| -> f64 { x.abs() });
    engine.register_fn("floor", |x: f64| -> f64 { x.floor() });
    engine.register_fn("ceil", |x: f64| -> f64 { x.ceil() });
    engine.register_fn("fract", |x: f64| -> f64 { x.fract() });
    engine.register_fn("clamp", |x: f64, lo: f64, hi: f64| -> f64 {
        x.clamp(lo, hi)
    });
    engine.register_fn("mix", |a: f64, b: f64, t: f64| -> f64 {
        a + (b - a) * t.clamp(0.0, 1.0)
    });
    engine.register_fn("min", |a: f64, b: f64| -> f64 { a.min(b) });
    engine.register_fn("max", |a: f64, b: f64| -> f64 { a.max(b) });

    engine.register_fn("rgb", |r: i64, g: i64, b: i64| -> rhai::Map {
        rgb_map(
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
        )
    });
    engine.register_fn("rgb_f", |r: f64, g: f64, b: f64| -> rhai::Map {
        rgb_map(
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    });
    engine.register_fn("from_hue", |h: f64| -> rhai::Map {
        let c = Rgb::from_hue(h as f32);
        rgb_map(c.r, c.g, c.b)
    });
    engine.register_fn("dim", |c: rhai::Map, scale: f64| -> rhai::Map {
        let scale = scale.clamp(0.0, 1.0);
        rgb_map(
            (map_get(&c, "r") as f64 * scale) as u8,
            (map_get(&c, "g") as f64 * scale) as u8,
            (map_get(&c, "b") as f64 * scale) as u8,
        )
    });
    engine.register_fn("blend", |a: rhai::Map, b: rhai::Map, t: f64| -> rhai::Map {
        let t = t.clamp(0.0, 1.0);
        let lerp = |x: i64, y: i64| (x as f64 + (y as f64 - x as f64) * t) as u8;
        rgb_map(
            lerp(map_get(&a, "r"), map_get(&b, "r")),
            lerp(map_get(&a, "g"), map_get(&b, "g")),
            lerp(map_get(&a, "b"), map_get(&b, "b")),
        )
    });

    engine
}

/// A Rhai script that drives a single color signal. Evaluated once per frame.
/// Scope: `time` (f64), `frame` (i64), `pi`.
/// Must return a color map via rgb(), rgb_f(), from_hue(), etc.
pub struct SignalScript {
    engine: Engine,
    ast: AST,
    pub live: LiveParam<Rgb>,
    frame: AtomicU64,
}

impl SignalScript {
    pub fn new(code: &str) -> Result<Arc<Self>, String> {
        let engine = make_engine();
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
            .eval_ast_with_scope::<Dynamic>(&mut scope, &self.ast)
        {
            Ok(val) => parse_color(val),
            Err(_) => Rgb::BLACK,
        };
        self.live.set_immediate(color);
    }
}
