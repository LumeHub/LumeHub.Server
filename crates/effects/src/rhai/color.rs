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

pub fn register(engine: &mut Engine) {
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

pub(super) fn heat_color(v: f32) -> Rgb {
    let v = v.clamp(0.0, 1.0);
    match v {
        h if h < 0.25 => Rgb::BLACK.lerp(Rgb::new(180, 0, 0), h * 4.0),
        h if h < 0.50 => Rgb::new(180, 0, 0).lerp(Rgb::new(255, 80, 0), (h - 0.25) * 4.0),
        h if h < 0.75 => Rgb::new(255, 80, 0).lerp(Rgb::new(255, 220, 0), (h - 0.50) * 4.0),
        h => Rgb::new(255, 220, 0).lerp(Rgb::new(255, 255, 255), (h - 0.75) * 4.0),
    }
}
