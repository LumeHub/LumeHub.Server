use domain::Rgb;
use engine::{Output, RenderCommand};
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

    tx.send(RenderCommand::SetColor(Rgb::new(255, 0, 0)))
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

    tx.send(RenderCommand::SetColor(Rgb::new(0, 255, 0)))
        .unwrap();
    wait_for(&shown, 1);
    tx.send(RenderCommand::SetOnOff(false)).unwrap();
    let frames = wait_for(&shown, 2);
    assert!(frames.last().unwrap().iter().all(|p| *p == Rgb::BLACK));
}

#[test]
fn execute_renders_effect_frames() {
    use engine::Effect;

    struct ThreeFrames;
    impl Effect for ThreeFrames {
        fn frames(&self, pixels: &[Rgb]) -> engine::FrameIter {
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

    tx.send(RenderCommand::Execute(Box::new(ThreeFrames)))
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
