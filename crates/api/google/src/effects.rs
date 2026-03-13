use domain::Rgb;
use engine::Effect;

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
        const FRAMES_PER_TRANSITION: usize = 50;
        let total_frames = (self.duration * 60) as usize;

        let mut frame_count = 0;
        Box::new(std::iter::from_fn(move || {
            if frame_count >= total_frames {
                return None;
            }
            let current_color_idx = (frame_count / FRAMES_PER_TRANSITION) % num_colors;
            let next_color_idx = (current_color_idx + 1) % num_colors;
            let progress =
                (frame_count % FRAMES_PER_TRANSITION) as f32 / FRAMES_PER_TRANSITION as f32;
            let color = Rgb::lerp(&colors[current_color_idx], colors[next_color_idx], progress);
            frame_count += 1;
            Some(vec![color; len])
        }))
    }
}

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
        let start_brightness = self.start_brightness;
        let target_color = self.target_color;

        let mut frame_count = 0;
        Box::new(std::iter::from_fn(move || {
            if frame_count >= total_frames {
                return None;
            }
            let progress = frame_count as f32 / total_frames as f32;
            let current_brightness = start_brightness as f32 * (1.0 - progress);
            let color = Rgb::lerp(&start_color, target_color, progress)
                .with_brightness(current_brightness.round() as u8);
            frame_count += 1;
            Some(vec![color; len])
        }))
    }
}

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
            let color =
                start_color.with_brightness((end_brightness as f32 * progress).round() as u8);
            frame_count += 1;
            Some(vec![color; len])
        }))
    }
}
