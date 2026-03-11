use std::sync::mpsc;

use super::{Effect, fade_in::FadeIn};

#[derive(Clone)]
pub struct EffectQueue {
    sender: mpsc::Sender<Box<dyn Effect + Send>>,
    crossfade_frames: usize,
}

impl EffectQueue {
    pub fn new(crossfade_ms: u32) -> (Self, mpsc::Receiver<Box<dyn Effect + Send>>) {
        let (sender, receiver) = mpsc::channel();
        let crossfade_frames = crossfade_ms as usize / 16;
        (
            Self {
                sender,
                crossfade_frames,
            },
            receiver,
        )
    }

    pub fn brightness_speed(&self) -> f32 {
        255.0 / self.crossfade_frames.max(1) as f32
    }

    pub fn enqueue(&self, effect: Box<dyn Effect + Send>) {
        let wrapped: Box<dyn Effect + Send> = if self.crossfade_frames > 0 {
            Box::new(FadeIn {
                inner: effect,
                frames: self.crossfade_frames,
            })
        } else {
            effect
        };
        self.sender.send(wrapped).unwrap();
    }
}
