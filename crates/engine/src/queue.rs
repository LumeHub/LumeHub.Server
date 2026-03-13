use std::sync::mpsc;

use crate::{Effect, RenderCommand};

#[derive(Clone)]
pub struct EffectQueue {
    sender: mpsc::Sender<RenderCommand>,
    crossfade_frames: usize,
}

impl EffectQueue {
    pub fn new(crossfade_ms: u32) -> (Self, mpsc::Receiver<RenderCommand>) {
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
        self.sender.send(RenderCommand::Execute(effect)).unwrap();
    }

    pub fn send(&self, cmd: RenderCommand) {
        self.sender.send(cmd).unwrap();
    }
}
