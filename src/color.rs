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

    pub fn interpolate(start: Rgb, end: Rgb, steps: usize) -> impl Iterator<Item = Rgb> {
        (0..=steps).map(move |i| start.lerp(end, i as f32 / steps as f32))
    }
}
