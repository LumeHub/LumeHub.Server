use rhai::Dynamic;

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
