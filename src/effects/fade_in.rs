use crate::color::Rgb;
use crate::effects::Effect;

pub struct FadeIn {
    pub inner: Box<dyn Effect + Send>,
    pub frames: usize,
}

impl Effect for FadeIn {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let from = pixels.to_vec();
        let total = self.frames;
        let mut inner = self.inner.frames(pixels);
        let mut progress = 0usize;

        Box::new(std::iter::from_fn(move || {
            let frame = inner.next()?;
            if progress >= total {
                return Some(frame);
            }
            let t = (progress + 1) as f32 / total as f32;
            progress += 1;
            Some(
                from.iter()
                    .zip(frame.iter())
                    .map(|(a, b)| a.lerp(*b, t))
                    .collect(),
            )
        }))
    }
}
