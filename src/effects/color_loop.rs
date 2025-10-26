use crate::color::Rgb;
use crate::effects::Effect;

pub struct ColorLoop {
    pub duration: u64,
    pub start_color: Rgb,
    pub colors: Vec<Rgb>,
}

impl Effect for ColorLoop {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let mut colors = self.colors.clone();
        colors.insert(0, self.start_color);

        let num_colors = colors.len();
        let frames_per_color = 50;
        let total_frames = (self.duration * 60) as usize;

        let mut frame_count = 0;
        Box::new(std::iter::from_fn(move || {
            if frame_count >= total_frames {
                return None;
            }

            let current_color_idx = (frame_count / frames_per_color) % num_colors;
            let next_color_idx = (current_color_idx + 1) % num_colors;
            let progress = (frame_count % frames_per_color) as f32 / frames_per_color as f32;

            let interpolated_color =
                Rgb::lerp(&colors[current_color_idx], colors[next_color_idx], progress);

            frame_count += 1;
            Some(vec![interpolated_color; len])
        }))
    }
}
