use crate::effects::Effect;
use domain::Rgb;

pub struct SolidColor {
    pub color: Rgb,
}

impl Effect for SolidColor {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let color = self.color;
        Box::new(std::iter::repeat_with(move || vec![color; len]))
    }
}
