use std::sync::mpsc;

use super::Effect;

#[derive(Clone)]
pub struct EffectQueue {
    sender: mpsc::Sender<Box<dyn Effect + Send>>,
}

impl EffectQueue {
    pub fn new() -> (Self, mpsc::Receiver<Box<dyn Effect + Send>>) {
        let (sender, receiver) = mpsc::channel();
        (Self { sender }, receiver)
    }

    pub fn enqueue(&self, effect: Box<dyn Effect + Send>) {
        self.sender.send(effect).unwrap();
    }
}
