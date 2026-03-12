use super::ActiveEffect;
use crate::{FrameIter, Output};
use domain::{DeviceState, Rgb};

pub fn transition_to_state(
    state: &DeviceState,
    current: &mut ActiveEffect,
    output: &mut dyn Output,
    crossfade_frames: usize,
) {
    let target = if state.on {
        state.color.with_brightness(state.brightness)
    } else {
        Rgb::BLACK
    };

    if crossfade_frames == 0 {
        output.pixels_mut().fill(target);
        output.show();
        *current = None;
        return;
    }

    let from = output.pixels().to_vec();
    *current = Some(fade_to_solid(from, target, crossfade_frames));
}

pub fn crossfade(from: Vec<Rgb>, mut inner: FrameIter, frames: usize) -> FrameIter {
    if frames == 0 {
        return inner;
    }
    let mut progress = 0;
    Box::new(std::iter::from_fn(move || {
        let frame = inner.next()?;
        if progress >= frames {
            return Some(frame);
        }
        let t = (progress + 1) as f32 / frames as f32;
        progress += 1;
        Some(
            from.iter()
                .zip(frame.iter())
                .map(|(a, b)| a.lerp(*b, t))
                .collect(),
        )
    }))
}

fn fade_to_solid(from: Vec<Rgb>, target: Rgb, frames: usize) -> FrameIter {
    let mut i = 0;
    Box::new(std::iter::from_fn(move || {
        if i >= frames {
            return None;
        }
        let t = (i + 1) as f32 / frames as f32;
        i += 1;
        Some(from.iter().map(|p| p.lerp(target, t)).collect())
    }))
}
