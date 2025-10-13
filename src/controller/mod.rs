pub mod color;
pub mod drivers;

pub trait Controller {
    fn len(&self) -> usize;
    fn show(&mut self);
    fn fill(&mut self, color: color::Rgb) {
        self.map(|_, _| color);
    }
    fn map<F>(&mut self, f: F)
    where
        F: FnMut(usize, color::Rgb) -> color::Rgb;
}
