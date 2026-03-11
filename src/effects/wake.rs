use crate::color::Rgb;
use crate::effects::Effect;

pub struct Wake {
    pub duration: u64,
    pub end_brightness: u8,
    pub start_color: Rgb,
}

impl Effect for Wake {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let total_frames = (self.duration * 60) as usize;
        let start_color = self.start_color;
        let end_brightness = self.end_brightness;

        let mut frame_count = 0;
        Box::new(std::iter::from_fn(move || {
            if frame_count >= total_frames {
                return None;
            }

            let progress = frame_count as f32 / total_frames as f32;
            let current_brightness = end_brightness as f32 * progress;
            let final_color = start_color.with_brightness(current_brightness.round() as u8);

            frame_count += 1;
            Some(vec![final_color; len])
        }))
    }
}
