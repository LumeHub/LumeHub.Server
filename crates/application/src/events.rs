use tokio::sync::broadcast;

use crate::SceneSnapshot;

#[derive(Clone)]
pub struct StateEventBus {
    tx: broadcast::Sender<SceneSnapshot>,
}

impl StateEventBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(16);
        Self { tx }
    }

    pub fn notify(&self, snapshot: SceneSnapshot) {
        let _ = self.tx.send(snapshot);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SceneSnapshot> {
        self.tx.subscribe()
    }
}

impl Default for StateEventBus {
    fn default() -> Self {
        Self::new()
    }
}
