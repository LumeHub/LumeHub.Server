use rhai::{Dynamic, Engine};

use domain::Rgb;

pub fn parse_color(val: Dynamic) -> Rgb {
    if val.is::<Rgb>() {
        return val.cast::<Rgb>();
    }
    if val.is_map() {
        let map = val.cast::<rhai::Map>();
        let get = |k: &str| map.get(k).and_then(|v| v.as_int().ok()).unwrap_or(0);
        return Rgb {
            r: get("r").clamp(0, 255) as u8,
            g: get("g").clamp(0, 255) as u8,
            b: get("b").clamp(0, 255) as u8,
        };
    }
    Rgb::BLACK
}

pub fn make_script_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(0);
    register_math(&mut engine);
    register_color(&mut engine);
    engine.register_fn("hash", hash);
    engine.register_fn("scanner_factor", scanner_factor);
    engine.register_fn("fire_color", fire_color);
    engine
}

pub fn make_signal_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(1_000);
    register_math(&mut engine);
    register_color(&mut engine);
    engine
}

fn heat_color(v: f32) -> Rgb {
    let v = v.clamp(0.0, 1.0);
    match v {
        h if h < 0.25 => Rgb::BLACK.lerp(Rgb::new(180, 0, 0), h * 4.0),
        h if h < 0.50 => Rgb::new(180, 0, 0).lerp(Rgb::new(255, 80, 0), (h - 0.25) * 4.0),
        h if h < 0.75 => Rgb::new(255, 80, 0).lerp(Rgb::new(255, 220, 0), (h - 0.50) * 4.0),
        h => Rgb::new(255, 220, 0).lerp(Rgb::new(255, 255, 255), (h - 0.75) * 4.0),
    }
}

fn hash(seed: i64) -> f64 {
    let x = seed as u64 ^ (seed as u64 >> 33);
    let x = x.wrapping_mul(0xff51afd7ed558ccd);
    let x = x ^ (x >> 33);
    x as f64 / u64::MAX as f64
}

fn scanner_factor(pixel: i64, len: i64, time: f64, speed: f64, width: f64) -> f64 {
    let period = 2.0 * (len - 1) as f64;
    let phase = (time * speed).rem_euclid(period);
    let head = if phase < len as f64 {
        phase
    } else {
        period - phase
    };
    let dist = (pixel as f64 - head).abs();
    (-dist * dist / (2.0 * width * width)).exp()
}

fn fire_color(pixel: i64, len: i64, frame: i64, flicker: f64, turbulence: f64, decay: f64) -> Rgb {
    let flicker_int = (flicker as i64).max(1);
    let slot = frame / flicker_int;
    let frac = (frame % flicker_int) as f64 / flicker.max(1.0);
    let n1 = hash((pixel * 7) ^ (slot * 97)) - 0.5;
    let n2 = hash((pixel * 7) ^ ((slot + 1) * 97)) - 0.5;
    let noise = (n1 + (n2 - n1) * frac.clamp(0.0, 1.0)) * turbulence;
    let base = (1.0 - pixel as f64 / len as f64).powf(decay);
    heat_color((base + noise).clamp(0.0, 1.0) as f32)
}

fn register_math(engine: &mut Engine) {
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
    engine.register_fn("to_float", |x: i64| -> f64 { x as f64 });
    engine.register_fn("to_int", |x: f64| -> i64 { x as i64 });
}

fn register_color(engine: &mut Engine) {
    engine.register_type_with_name::<Rgb>("Rgb");
    engine.register_get("r", |c: &mut Rgb| c.r as i64);
    engine.register_get("g", |c: &mut Rgb| c.g as i64);
    engine.register_get("b", |c: &mut Rgb| c.b as i64);
    engine.register_fn("rgb", |r: i64, g: i64, b: i64| -> Rgb {
        Rgb::new(
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
        )
    });
    engine.register_fn("rgb_f", |r: f64, g: f64, b: f64| -> Rgb {
        Rgb::new(
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    });
    engine.register_fn("from_hue", |h: f64| -> Rgb { Rgb::from_hue(h as f32) });
    engine.register_fn("heat_color", |v: f64| -> Rgb { heat_color(v as f32) });
    engine.register_fn("dim", |c: Rgb, scale: f64| -> Rgb { c.dim(scale as f32) });
    engine.register_fn("blend", |a: Rgb, b: Rgb, t: f64| -> Rgb {
        a.lerp(b, t.clamp(0.0, 1.0) as f32)
    });
    engine.register_fn("add_colors", |a: Rgb, b: Rgb| -> Rgb { a + b });
}
