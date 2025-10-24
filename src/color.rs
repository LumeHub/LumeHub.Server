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

    pub fn with_brightness(&self, brightness: u8) -> Self {
        let scale = brightness as f32 / 100.0;
        Rgb {
            r: (self.r as f32 * scale) as u8,
            g: (self.g as f32 * scale) as u8,
            b: (self.b as f32 * scale) as u8,
        }
    }

    pub fn to_spectrum(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    pub fn from_spectrum_rgb(spectrum_rgb: u32) -> Self {
        let r = ((spectrum_rgb >> 16) & 0xFF) as u8;
        let g = ((spectrum_rgb >> 8) & 0xFF) as u8;
        let b = (spectrum_rgb & 0xFF) as u8;
        Rgb { r, g, b }
    }

    pub fn from_temperature_k(kelvin: u32) -> Self {
        let temp = kelvin as f32 / 100.0;
        let mut r: f32;
        let mut g: f32;
        let mut b: f32;

        // Red
        if temp < 66.0 {
            r = 255.0;
        } else {
            r = temp - 60.0;
            r = 329.69873 * f32::powf(r, -0.13320476);
            r = r.clamp(0.0, 255.0);
        }

        // Green
        if temp < 66.0 {
            g = temp;
            g = 99.4708 * f32::ln(g) - 161.11957;
            g = g.clamp(0.0, 255.0);
        } else {
            g = temp - 60.0;
            g = 288.12216 * f32::powf(g, -0.075514846);
            g = g.clamp(0.0, 255.0);
        }

        // Blue
        if temp >= 66.0 {
            b = 255.0;
        } else if temp < 19.0 {
            b = 0.0;
        } else {
            b = temp - 10.0;
            b = 138.51773 * f32::ln(b) - 305.0448;
            b = b.clamp(0.0, 255.0);
        }

        Rgb {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}
