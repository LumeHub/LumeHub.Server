use crate::color::Rgb;
use crate::controller::Controller;
use crate::effects;
use std::{thread, time::Duration};

pub fn spawn(
    mut led: Box<dyn Controller>,
    rx: std::sync::mpsc::Receiver<Box<dyn effects::Effect + Send>>,
) {
    thread::spawn(move || {
        let mut current_effect_iterator: Option<Box<dyn Iterator<Item = Vec<Rgb>> + Send>> = None;

        loop {
            // Prioritize new effects from the queue
            if let Ok(new_effect) = rx.try_recv() {
                current_effect_iterator = Some(new_effect.frames(led.as_pixel_slice()));
            }

            // Process the current effect, if any
            if let Some(ref mut effect_iter) = current_effect_iterator {
                if let Some(frame) = effect_iter.next() {
                    led.as_pixel_slice_mut().copy_from_slice(&frame);
                    led.show();
                } else {
                    // Effect finished, clear it to allow waiting for a new one
                    current_effect_iterator = None;
                }
            } else {
                // No effect is active; block until a new one is received
                if let Ok(initial_effect) = rx.recv() {
                    current_effect_iterator = Some(initial_effect.frames(led.as_pixel_slice()));
                }
            }

            // Control frame rate
            thread::sleep(Duration::from_millis(10));
        }
    });
}
