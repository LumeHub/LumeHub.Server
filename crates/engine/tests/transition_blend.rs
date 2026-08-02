use domain::{Rgb, TransitionCurve};
use engine::{FrameIter, blend, solid_frames};

fn white() -> Rgb {
    Rgb::new(255, 255, 255)
}

fn black() -> Rgb {
    Rgb::BLACK
}

fn finite(frames: Vec<Vec<Rgb>>) -> FrameIter {
    Box::new(frames.into_iter())
}

#[test]
fn zero_frames_passes_through_to_iter_unchanged() {
    let from = solid_frames(vec![white(); 1]);
    let to = finite(vec![vec![black(); 1], vec![black(); 1]]);
    let collected: Vec<_> = blend(from, to, 0, TransitionCurve::Linear).collect();
    assert_eq!(collected, vec![vec![black(); 1], vec![black(); 1]]);
}

#[test]
fn linear_blend_lerps_endpoints_proportionally() {
    let from = solid_frames(vec![white(); 1]);
    let to = solid_frames(vec![black(); 1]);
    let mut iter = blend(from, to, 4, TransitionCurve::Linear);
    let first = iter.next().unwrap();
    assert_eq!(first[0], white().lerp(black(), 0.25));
    let second = iter.next().unwrap();
    assert_eq!(second[0], white().lerp(black(), 0.5));
}

#[test]
fn last_blend_frame_reaches_to_value_at_progress_n() {
    let from = solid_frames(vec![white(); 1]);
    let to = solid_frames(vec![black(); 1]);
    let mut iter = blend(from, to, 3, TransitionCurve::Linear);
    let mut last = vec![white(); 1];
    for _ in 0..3 {
        last = iter.next().unwrap();
    }
    assert_eq!(last, vec![black(); 1]);
}

#[test]
fn after_blend_completes_iter_passes_through_to_values() {
    let from = solid_frames(vec![white(); 1]);
    let to = finite(vec![
        vec![Rgb::new(10, 10, 10); 1],
        vec![Rgb::new(20, 20, 20); 1],
        vec![Rgb::new(30, 30, 30); 1],
    ]);
    let mut iter = blend(from, to, 1, TransitionCurve::Linear);
    iter.next();
    assert_eq!(iter.next().unwrap(), vec![Rgb::new(20, 20, 20); 1]);
    assert_eq!(iter.next().unwrap(), vec![Rgb::new(30, 30, 30); 1]);
}

#[test]
fn iter_ends_when_to_iter_ends() {
    let from = solid_frames(vec![white(); 1]);
    let to = finite(vec![vec![black(); 1]; 2]);
    let mut iter = blend(from, to, 5, TransitionCurve::Linear);
    iter.next();
    iter.next();
    assert!(iter.next().is_none());
}

#[test]
fn ease_in_curve_starts_closer_to_from_than_linear() {
    let from = solid_frames(vec![white(); 1]);
    let to = solid_frames(vec![black(); 1]);
    let linear_first = blend(
        solid_frames(vec![white(); 1]),
        solid_frames(vec![black(); 1]),
        4,
        TransitionCurve::Linear,
    )
    .next()
    .unwrap()[0]
        .r;
    let ease_first = blend(from, to, 4, TransitionCurve::EaseIn).next().unwrap()[0].r;
    assert!(
        ease_first > linear_first,
        "ease_in should remain whiter (higher r) at t=1/4 than linear"
    );
}

#[test]
fn exhausted_from_iter_holds_last_frame_for_remaining_blend() {
    let from: FrameIter = Box::new(std::iter::once(vec![white(); 1]));
    let to = solid_frames(vec![black(); 1]);
    let mut iter = blend(from, to, 3, TransitionCurve::Linear);
    for _ in 0..3 {
        assert!(iter.next().is_some());
    }
}
