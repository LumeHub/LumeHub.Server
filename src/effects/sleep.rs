use crate::color::Rgb;
use crate::effects::Effect;

pub struct Sleep {
    pub duration: u64,
    pub start_brightness: u8,
    pub target_color: Rgb,
}

impl Effect for Sleep {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let total_frames = (self.duration * 60) as usize;
        let start_color = pixels[0];
        let effect_start_brightness = self.start_brightness;
        let effect_target_color = self.target_color;

        let mut frame_count = 0;
        Box::new(std::iter::from_fn(move || {
            if frame_count >= total_frames {
                return None;
            }

            let progress = frame_count as f32 / total_frames as f32;
            let current_brightness = effect_start_brightness as f32 * (1.0 - progress);

            let interpolated_color = Rgb::lerp(&start_color, effect_target_color, progress);
            let final_color = interpolated_color.with_brightness(current_brightness.round() as u8);

            frame_count += 1;
            Some(vec![final_color; len])
        }))
    }
}
