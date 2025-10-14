use crate::color::Rgb;

pub mod fade_color;

pub trait Effect {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>>>;
}
