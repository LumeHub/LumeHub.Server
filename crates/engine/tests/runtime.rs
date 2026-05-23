use domain::{Rgb, TransitionCurve, TransitionSpec};
use engine::{Effect, FrameIter, Output, RenderCommand};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct MockOutput {
    pixels: Vec<Rgb>,
    shown: Arc<Mutex<Vec<Vec<Rgb>>>>,
}

impl MockOutput {
    fn new(len: usize) -> (Self, Arc<Mutex<Vec<Vec<Rgb>>>>) {
        let shown = Arc::new(Mutex::new(vec![]));
        (
            Self {
                pixels: vec![Rgb::BLACK; len],
                shown: Arc::clone(&shown),
            },
            shown,
        )
    }
}

impl Output for MockOutput {
    fn pixels(&self) -> &[Rgb] {
        &self.pixels
    }
    fn pixels_mut(&mut self) -> &mut [Rgb] {
        &mut self.pixels
    }
    fn show(&mut self) {
        self.shown.lock().unwrap().push(self.pixels.clone());
    }
}

fn wait_for(shown: &Arc<Mutex<Vec<Vec<Rgb>>>>, count: usize) -> Vec<Vec<Rgb>> {
    let deadline = Instant::now() + Duration::from_millis(200);
    loop {
        let frames = shown.lock().unwrap().clone();
        if frames.len() >= count {
            return frames;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {count} frames"
        );
        thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn set_color_renders_solid_frame() {
    let (mock, shown) = MockOutput::new(4);
    let (tx, rx) = mpsc::channel();
    engine::spawn(Box::new(mock), rx);

    tx.send(RenderCommand::SetColor(
        Rgb::new(255, 0, 0),
        TransitionSpec::INSTANT,
    ))
    .unwrap();
    let frames = wait_for(&shown, 1);
    assert!(
        frames
            .last()
            .unwrap()
            .iter()
            .all(|p| *p == Rgb::new(255, 0, 0))
    );
}

#[test]
fn set_on_false_renders_black() {
    let (mock, shown) = MockOutput::new(4);
    let (tx, rx) = mpsc::channel();
    engine::spawn(Box::new(mock), rx);

    tx.send(RenderCommand::SetColor(
        Rgb::new(0, 255, 0),
        TransitionSpec::INSTANT,
    ))
    .unwrap();
    wait_for(&shown, 1);
    tx.send(RenderCommand::SetOnOff(false, TransitionSpec::INSTANT))
        .unwrap();
    let frames = wait_for(&shown, 2);
    assert!(frames.last().unwrap().iter().all(|p| *p == Rgb::BLACK));
}

#[test]
fn execute_renders_effect_frames() {
    struct ThreeFrames;
    impl Effect for ThreeFrames {
        fn frames(&self, pixels: &[Rgb]) -> FrameIter {
            let len = pixels.len();
            let colors = [
                Rgb::new(255, 0, 0),
                Rgb::new(0, 255, 0),
                Rgb::new(0, 0, 255),
            ];
            let mut i = 0;
            Box::new(std::iter::from_fn(move || {
                if i < 3 {
                    let frame = vec![colors[i]; len];
                    i += 1;
                    Some(frame)
                } else {
                    None
                }
            }))
        }
    }

    let (mock, shown) = MockOutput::new(4);
    let (tx, rx) = mpsc::channel();
    engine::spawn(Box::new(mock), rx);

    tx.send(RenderCommand::Execute(
        Box::new(ThreeFrames),
        TransitionSpec::INSTANT,
    ))
    .unwrap();
    let frames = wait_for(&shown, 3);
    let expected = [
        Rgb::new(255, 0, 0),
        Rgb::new(0, 255, 0),
        Rgb::new(0, 0, 255),
    ];
    for (frame, color) in frames.iter().zip(expected.iter()) {
        assert!(frame.iter().all(|p| p == color));
    }
}

#[test]
fn execute_with_fade_blends_first_frame_between_current_and_target() {
    struct Solid(Rgb);
    impl Effect for Solid {
        fn frames(&self, pixels: &[Rgb]) -> FrameIter {
            let color = self.0;
            let len = pixels.len();
            Box::new(std::iter::repeat_with(move || vec![color; len]))
        }
    }

    let (mock, shown) = MockOutput::new(2);
    let (tx, rx) = mpsc::channel();
    engine::spawn(Box::new(mock), rx);

    tx.send(RenderCommand::SetColor(
        Rgb::new(255, 255, 255),
        TransitionSpec::INSTANT,
    ))
    .unwrap();
    wait_for(&shown, 1);

    // 4-frame fade from white to black (4 * 16ms = 64ms)
    let fade = TransitionSpec::new(64, TransitionCurve::Linear);
    tx.send(RenderCommand::Execute(Box::new(Solid(Rgb::BLACK)), fade))
        .unwrap();
    let frames = wait_for(&shown, 2);
    let blended = &frames[1];
    assert!(blended[0].r < 255 && blended[0].r > 0);
}
