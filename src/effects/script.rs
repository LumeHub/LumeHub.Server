use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rhai::{AST, Dynamic, Engine, Map, Scope};
use serde::Deserialize;
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::composite::live_param::{PRIMARY_COLOR, SECONDARY_COLOR};
use crate::effects::composite::{LayerEffect, ParameterBus};
use crate::effects::registry::{EffectBuildError, EffectRegistry};

// Any numeric key in params besides `code` becomes a named f64 scope variable.
// e.g. `speed = 0.04` in TOML → `speed` available in the script.
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

fn rgb_map(r: u8, g: u8, b: u8) -> Map {
    let mut map = Map::new();
    map.insert("r".into(), Dynamic::from(r as i64));
    map.insert("g".into(), Dynamic::from(g as i64));
    map.insert("b".into(), Dynamic::from(b as i64));
    map
}

fn map_get(m: &Map, k: &str) -> i64 {
    m.get(k).and_then(|v| v.as_int().ok()).unwrap_or(0)
}

fn heat_color(v: f32) -> Rgb {
    let v = v.clamp(0.0, 1.0);
    match v {
        h if h < 0.25 => Rgb::BLACK.lerp(Rgb::new(180, 0, 0), h * 4.0),
        h if h < 0.5 => Rgb::new(180, 0, 0).lerp(Rgb::new(255, 80, 0), (h - 0.25) * 4.0),
        h if h < 0.75 => Rgb::new(255, 80, 0).lerp(Rgb::new(255, 220, 0), (h - 0.5) * 4.0),
        h => Rgb::new(255, 220, 0).lerp(Rgb::new(255, 255, 255), (h - 0.75) * 4.0),
    }
}

fn make_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(100_000);

    // Math — registered as free functions so scripts write sin(x) not x.sin()
    engine.register_fn("sin", |x: f64| -> f64 { x.sin() });
    engine.register_fn("cos", |x: f64| -> f64 { x.cos() });
    engine.register_fn("tan", |x: f64| -> f64 { x.tan() });
    engine.register_fn("sqrt", |x: f64| -> f64 { x.sqrt() });
    engine.register_fn("pow", |x: f64, y: f64| -> f64 { x.powf(y) });
    engine.register_fn("exp", |x: f64| -> f64 { x.exp() });
    engine.register_fn("log", |x: f64| -> f64 { x.ln() });
    engine.register_fn("abs", |x: f64| -> f64 { x.abs() });
    engine.register_fn("abs", |x: i64| -> i64 { x.abs() });
    engine.register_fn("floor", |x: f64| -> f64 { x.floor() });
    engine.register_fn("ceil", |x: f64| -> f64 { x.ceil() });
    engine.register_fn("round", |x: f64| -> f64 { x.round() });
    engine.register_fn("fract", |x: f64| -> f64 { x.fract() });
    engine.register_fn("clamp", |x: f64, lo: f64, hi: f64| -> f64 {
        x.clamp(lo, hi)
    });
    engine.register_fn("mix", |a: f64, b: f64, t: f64| -> f64 {
        a + (b - a) * t.clamp(0.0, 1.0)
    });
    engine.register_fn("min", |a: f64, b: f64| -> f64 { a.min(b) });
    engine.register_fn("max", |a: f64, b: f64| -> f64 { a.max(b) });
    // Type conversions — callable as method or free function: age.to_float(), to_float(age)
    engine.register_fn("to_float", |x: i64| -> f64 { x as f64 });
    engine.register_fn("to_int", |x: f64| -> i64 { x as i64 });

    // Color constructors — the script returns one of these as its final expression
    engine.register_fn("rgb", |r: i64, g: i64, b: i64| -> Map {
        rgb_map(
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
        )
    });
    engine.register_fn("rgb_f", |r: f64, g: f64, b: f64| -> Map {
        rgb_map(
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    });
    engine.register_fn("from_hue", |h: f64| -> Map {
        let c = Rgb::from_hue(h as f32);
        rgb_map(c.r, c.g, c.b)
    });
    engine.register_fn("heat_color", |v: f64| -> Map {
        let c = heat_color(v as f32);
        rgb_map(c.r, c.g, c.b)
    });

    // Color combinators — operate on color maps returned by the constructors above
    engine.register_fn("dim", |c: Map, scale: f64| -> Map {
        let scale = scale.clamp(0.0, 1.0);
        rgb_map(
            (map_get(&c, "r") as f64 * scale) as u8,
            (map_get(&c, "g") as f64 * scale) as u8,
            (map_get(&c, "b") as f64 * scale) as u8,
        )
    });
    engine.register_fn("blend", |a: Map, b: Map, t: f64| -> Map {
        let t = t.clamp(0.0, 1.0);
        let lerp = |x: i64, y: i64| (x as f64 + (y as f64 - x as f64) * t) as u8;
        rgb_map(
            lerp(map_get(&a, "r"), map_get(&b, "r")),
            lerp(map_get(&a, "g"), map_get(&b, "g")),
            lerp(map_get(&a, "b"), map_get(&b, "b")),
        )
    });
    engine.register_fn("add_colors", |a: Map, b: Map| -> Map {
        let add = |x: i64, y: i64| (x + y).clamp(0, 255) as u8;
        rgb_map(
            add(map_get(&a, "r"), map_get(&b, "r")),
            add(map_get(&a, "g"), map_get(&b, "g")),
            add(map_get(&a, "b"), map_get(&b, "b")),
        )
    });

    // Pseudo-random float 0.0–1.0 from an integer seed
    engine.register_fn("hash", |seed: i64| -> f64 {
        let x = seed as u64 ^ (seed as u64 >> 33);
        let x = x.wrapping_mul(0xff51afd7ed558ccd);
        let x = x ^ (x >> 33);
        x as f64 / u64::MAX as f64
    });

    engine
}

fn parse_color(val: Dynamic) -> Rgb {
    if val.is_map() {
        let map = val.cast::<Map>();
        let r = map_get(&map, "r").clamp(0, 255) as u8;
        let g = map_get(&map, "g").clamp(0, 255) as u8;
        let b = map_get(&map, "b").clamp(0, 255) as u8;
        Rgb { r, g, b }
    } else {
        Rgb::BLACK
    }
}

impl LayerEffect for Script {
    fn render(&self, len: usize) -> Vec<Rgb> {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);

        // Build a base scope with all per-frame constants once, clone it per pixel.
        let mut base = Scope::new();
        base.push("len", len as i64);
        base.push("time", frame as f64);
        base.push("frame", frame as i64);
        base.push("pi", std::f64::consts::PI);

        // Push every bus color signal as a named Map variable — any signal set via
        // set_signal is immediately visible by its name in the script.
        // Also compute pr/pg/pb and sr/sg/sb convenience floats.
        let mut pr = 1.0_f64;
        let mut pg = 1.0_f64;
        let mut pb = 1.0_f64;
        let mut sr = 0.0_f64;
        let mut sg = 0.0_f64;
        let mut sb = 0.0_f64;
        for (name, param) in self.bus.all_colors() {
            let c = param.get();
            base.push(name.as_str(), rgb_map(c.r, c.g, c.b));
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
        base.push("pr", pr);
        base.push("pg", pg);
        base.push("pb", pb);
        base.push("sr", sr);
        base.push("sg", sg);
        base.push("sb", sb);

        for (name, val) in &self.extra_vars {
            base.push(name.as_str(), *val);
        }

        (0..len)
            .map(|i| {
                let mut scope = base.clone();
                scope.push("pixel", i as i64);
                scope.push("t", i as f64 / (len as f64 - 1.0).max(1.0));

                match self
                    .engine
                    .eval_ast_with_scope::<Dynamic>(&mut scope, &self.ast)
                {
                    Ok(val) => parse_color(val),
                    Err(_) => Rgb::BLACK,
                }
            })
            .collect()
    }
}

pub fn register(registry: &mut EffectRegistry) {
    registry.register_layer("script", |params: Value, bus: Arc<ParameterBus>| {
        let p: ScriptParams =
            serde_json::from_value(params).map_err(|e| EffectBuildError(e.to_string()))?;

        let mut extra_vars: Vec<(String, f64)> = p
            .vars
            .into_iter()
            .filter_map(|(k, v)| v.as_f64().map(|f| (k, f)))
            .collect();
        // speed is implicitly available in every script; default 1.0
        if !extra_vars.iter().any(|(k, _)| k == "speed") {
            extra_vars.push(("speed".to_string(), 1.0));
        }

        let engine = make_engine();
        let full_code = format!("{}\n{}", bus.prelude, p.code);
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
    });
}
