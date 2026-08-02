use domain::{Rgb, TransitionCurve};

use crate::FrameIter;

pub fn blend(
    mut from: FrameIter,
    mut to: FrameIter,
    frames: u32,
    curve: TransitionCurve,
) -> FrameIter {
    if frames == 0 {
        return to;
    }

    let mut progress: u32 = 0;
    let mut last_from: Option<Vec<Rgb>> = None;
    Box::new(std::iter::from_fn(move || {
        let to_frame = to.next()?;
        if progress >= frames {
            return Some(to_frame);
        }
        let from_frame = from.next().or_else(|| last_from.clone())?;
        last_from = Some(from_frame.clone());
        progress += 1;
        let t = curve.weight(progress as f32 / frames as f32);
        Some(
            from_frame
                .iter()
                .zip(to_frame.iter())
                .map(|(a, b)| a.lerp(*b, t))
                .collect(),
        )
    }))
}
