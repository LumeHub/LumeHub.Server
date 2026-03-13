use rhai::Engine;

pub fn register(engine: &mut Engine) {
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
    engine.register_fn("wave", |x: f64| -> f64 { x.sin() * 0.5 + 0.5 });
    engine.register_fn("tri", |x: f64| -> f64 {
        let p = (x / std::f64::consts::PI).rem_euclid(2.0);
        if p < 1.0 { p } else { 2.0 - p }
    });
    engine.register_fn("saw", |x: f64| -> f64 {
        (x / (2.0 * std::f64::consts::PI)).rem_euclid(1.0)
    });
    engine.register_fn("smoothstep", |lo: f64, hi: f64, x: f64| -> f64 {
        let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    });
    engine.register_fn("remap", |x: f64, a: f64, b: f64, c: f64, d: f64| -> f64 {
        c + (x - a) / (b - a) * (d - c)
    });
}
