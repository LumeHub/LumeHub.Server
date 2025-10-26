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
                current_frame_iterator = None; // Invalidate old iterator
            }

            if let (None, Some(effect)) = (current_frame_iterator.as_ref(), current_effect.as_ref())
            {
                current_frame_iterator = Some(effect.frames(led.as_pixel_slice()));
            }

            if let Some(ref mut frame_iter) = current_frame_iterator {
                if let Some(frame) = frame_iter.next() {
                    led.as_pixel_slice_mut().copy_from_slice(&frame);
                    led.show();
                } else {
                    current_effect = None;
                    current_frame_iterator = None;
                }
            } else if let Ok(initial_effect) = rx.recv() {
                current_effect = Some(initial_effect);
                current_frame_iterator = Some(
                    current_effect
                        .as_ref()
                        .unwrap()
                        .frames(led.as_pixel_slice()),
                );
            }
            // Control frame rate
            thread::sleep(Duration::from_millis(10));
        }
    });
}
