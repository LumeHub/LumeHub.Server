use rhai::Engine;

use domain::Rgb;

use super::color::heat_color;

pub fn register(engine: &mut Engine) {
    engine.register_fn("hash", hash);
    engine.register_fn("scanner_factor", scanner_factor);
    engine.register_fn("fire_color", fire_color);
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
