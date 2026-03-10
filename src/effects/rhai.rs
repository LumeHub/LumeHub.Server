use rhai::{Dynamic, Map};

use crate::color::Rgb;

pub fn rgb_map(r: u8, g: u8, b: u8) -> Map {
    let mut map = Map::new();
    map.insert("r".into(), Dynamic::from(r as i64));
    map.insert("g".into(), Dynamic::from(g as i64));
    map.insert("b".into(), Dynamic::from(b as i64));
    map
}

pub fn map_get(m: &Map, k: &str) -> i64 {
    m.get(k).and_then(|v| v.as_int().ok()).unwrap_or(0)
}

pub fn parse_color(val: Dynamic) -> Rgb {
    if val.is_map() {
        let map = val.cast::<Map>();
        Rgb {
            r: map_get(&map, "r").clamp(0, 255) as u8,
            g: map_get(&map, "g").clamp(0, 255) as u8,
            b: map_get(&map, "b").clamp(0, 255) as u8,
        }
    } else {
        Rgb::BLACK
    }
}
