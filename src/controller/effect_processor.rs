use crate::controller::Controller;
use crate::effects;
use domain::Rgb;
use std::thread;
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(16);

pub fn spawn(
    mut led: Box<dyn Controller>,
    rx: std::sync::mpsc::Receiver<Box<dyn effects::Effect + Send>>,
) {
    thread::spawn(move || {
        let mut current_effect: Option<Box<dyn effects::Effect + Send>> = None;
        let mut current_frame_iterator: Option<
            Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static>,
        > = None;

        loop {
            let frame_start = Instant::now();

            if let Ok(new_effect) = rx.try_recv() {
                current_effect = Some(new_effect);
                current_frame_iterator = None;
            }

            if let (None, Some(effect)) = (current_frame_iterator.as_ref(), current_effect.as_ref())
            {
                current_frame_iterator = Some(effect.frames(led.pixels()));
            }

            match current_frame_iterator.as_mut().map(|iter| iter.next()) {
                None => {
                    if let Ok(effect) = rx.recv() {
                        current_effect = Some(effect);
                    }
                    continue;
                }
                Some(None) => {
                    current_effect = None;
                    current_frame_iterator = None;
                }
                Some(Some(frame)) => {
                    if frame.as_slice() != led.pixels() {
                        led.pixels_mut().copy_from_slice(&frame);
                        led.show();
                    }
                }
            }

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                thread::sleep(FRAME_DURATION - elapsed);
            }
        }
    });
}
