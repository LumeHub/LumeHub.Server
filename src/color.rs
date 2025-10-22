#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };

    pub fn lerp(&self, other: Rgb, t: f32) -> Self {
        let mix = |x1, x2| (x1 as f32 * (1.0 - t) + x2 as f32 * t).round() as u8;
        Rgb {
            r: mix(self.r, other.r),
            g: mix(self.g, other.g),
            b: mix(self.b, other.b),
        }
    }

    pub fn interpolate(start: Rgb, end: Rgb, step_size: u8) -> impl Iterator<Item = Rgb> {
        let max_distance = start.distance(end);
        let steps = ((max_distance as f32 / step_size as f32).ceil() as usize).max(1);
        (0..steps).map(move |i| {
            if steps == 1 {
                return end;
            }
            let t = i as f32 / (steps - 1) as f32;
            start.lerp(end, t)
        })
    }

    fn distance(&self, other: Rgb) -> u8 {
        let dr = (self.r as i16 - other.r as i16).abs();
        let dg = (self.g as i16 - other.g as i16).abs();
        let db = (self.b as i16 - other.b as i16).abs();
        dr.max(dg).max(db) as u8
    }
}
