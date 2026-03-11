use crate::color::Rgb;
use crate::effects::Effect;

pub struct Halt;

impl Effect for Halt {
    fn frames(&self, _pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        Box::new(std::iter::empty())
    }
}
