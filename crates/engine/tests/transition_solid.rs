use domain::{Rgb, TransitionCurve};
use engine::{fade_to_solid, solid_frames};

fn red() -> Rgb {
    Rgb::new(255, 0, 0)
}

fn black() -> Rgb {
    Rgb::BLACK
}

#[test]
fn solid_frames_repeats_the_same_pixel_buffer() {
    let mut iter = solid_frames(vec![red(); 3]);
    for _ in 0..5 {
        assert_eq!(iter.next().unwrap(), vec![red(); 3]);
    }
}

#[test]
fn fade_to_solid_with_zero_frames_yields_target_once_then_ends() {
    let from = solid_frames(vec![red(); 2]);
    let mut iter = fade_to_solid(from, vec![black(); 2], 0, TransitionCurve::Linear);
    assert_eq!(iter.next().unwrap(), vec![black(); 2]);
    assert!(iter.next().is_none());
}

#[test]
fn fade_to_solid_emits_exactly_n_frames_then_ends() {
    let from = solid_frames(vec![red(); 1]);
    let mut iter = fade_to_solid(from, vec![black(); 1], 4, TransitionCurve::Linear);
    for _ in 0..4 {
        assert!(iter.next().is_some());
    }
    assert!(iter.next().is_none());
}

#[test]
fn fade_to_solid_final_frame_equals_target_with_linear_curve() {
    let from = solid_frames(vec![red(); 1]);
    let mut iter = fade_to_solid(from, vec![black(); 1], 4, TransitionCurve::Linear);
    let last = (0..4).filter_map(|_| iter.next()).last().unwrap();
    assert_eq!(last, vec![black(); 1]);
}

#[test]
fn fade_to_solid_intermediate_frame_lies_between_endpoints_with_linear_curve() {
    let from = solid_frames(vec![red(); 1]);
    let mut iter = fade_to_solid(from, vec![black(); 1], 4, TransitionCurve::Linear);
    let first = iter.next().unwrap();
    assert!(first[0].r > 0 && first[0].r < 255);
}

#[test]
fn fade_to_solid_with_exhausted_from_iter_holds_last_frame() {
    let from: engine::FrameIter = Box::new(std::iter::once(vec![red(); 1]));
    let mut iter = fade_to_solid(from, vec![black(); 1], 3, TransitionCurve::Linear);
    for _ in 0..3 {
        assert!(iter.next().is_some());
    }
    assert!(iter.next().is_none());
}
