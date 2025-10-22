use crate::color::Rgb;
use crate::effects::Effect;

pub struct ZoneInterpolation {
    pub color1: Rgb,
    pub color2: Rgb,
    pub interpolation_length: usize,
    pub midpoint: usize,
}

impl Effect for ZoneInterpolation {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>>> {
        let len = pixels.len();
        let mut frame = vec![Rgb::BLACK; len];

        let interp_start = self.midpoint.saturating_sub(self.interpolation_length / 2);
        let interp_end = self.midpoint + self.interpolation_length / 2;

        (0..len).for_each(|i| {
            if i < interp_start {
                frame[i] = self.color1;
            } else if i >= interp_end {
                frame[i] = self.color2;
            } else {
                let t = (i - interp_start) as f32 / self.interpolation_length as f32;
                frame[i] = self.color1.lerp(self.color2, t);
            }
        });

        Box::new(std::iter::once(frame))
    }
}
