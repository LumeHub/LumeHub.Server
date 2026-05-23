use domain::{Rgb, TransitionCurve};

use crate::FrameIter;

pub fn solid_frames(pixels: Vec<Rgb>) -> FrameIter {
    Box::new(std::iter::repeat(pixels))
}

pub fn fade_to_solid(
    mut from: FrameIter,
    target: Vec<Rgb>,
    frames: u32,
    curve: TransitionCurve,
) -> FrameIter {
    if frames == 0 {
        return Box::new(std::iter::once(target));
    }

    let mut progress: u32 = 0;
    let mut last_from: Option<Vec<Rgb>> = None;
    Box::new(std::iter::from_fn(move || {
        if progress >= frames {
            return None;
        }
        let from_frame = from.next().or_else(|| last_from.clone())?;
        last_from = Some(from_frame.clone());
        progress += 1;
        let t = curve.weight(progress as f32 / frames as f32);
        Some(
            from_frame
                .iter()
                .zip(target.iter())
                .map(|(a, b)| a.lerp(*b, t))
                .collect(),
        )
    }))
}
