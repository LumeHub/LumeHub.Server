use domain::Rgb;

pub type FrameIter = Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static>;

pub trait Effect: Send {
    fn frames(&self, pixels: &[Rgb]) -> FrameIter;
}
