use domain::Rgb;

pub trait Output: Send {
    fn pixels(&self) -> &[Rgb];
    fn pixels_mut(&mut self) -> &mut [Rgb];
    fn show(&mut self);
}
