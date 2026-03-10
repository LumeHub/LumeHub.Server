use crate::controller::Controller;
use crate::effects;
use std::{thread, time::Duration};

pub fn spawn(
    mut led: Box<dyn Controller>,
    rx: std::sync::mpsc::Receiver<Box<dyn effects::Effect + Send>>,
) {
    thread::spawn(move || {
        let mut current_effect: Option<Box<dyn effects::Effect + Send>> = None;
        let mut current_frame_iterator: Option<
            Box<dyn Iterator<Item = Vec<crate::color::Rgb>> + Send + 'static>,
        > = None;

        loop {
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
                    if frame != led.pixels() {
                        led.pixels_mut().copy_from_slice(&frame);
                        led.show();
                    }
                }
            }

            thread::sleep(Duration::from_millis(10));
        }
    });
}
