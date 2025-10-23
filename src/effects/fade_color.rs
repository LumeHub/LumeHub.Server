use crate::color::Rgb;
use crate::effects::Effect;

pub struct FadeColor {
    pub color: Rgb,
}

impl Effect for FadeColor {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send> {
        let start_color = pixels[0];
        let len = pixels.len();
        Box::new(Rgb::interpolate(start_color, self.color, 5).map(move |c| vec![c; len]))
    }
}
