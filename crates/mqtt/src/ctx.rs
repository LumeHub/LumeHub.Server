use std::sync::{Arc, Mutex};

use application::SceneRuntime;
use effects::BuiltinEffect;
use rumqttc::AsyncClient;
use store::Store;

use crate::config::MqttConfig;
use crate::topics::Topics;

pub struct Ctx {
    pub client: AsyncClient,
    pub topics: Topics,
    pub config: MqttConfig,
    pub runtime: Arc<dyn SceneRuntime>,
    pub store: Arc<Store>,
    pub builtins: Arc<Vec<BuiltinEffect>>,
    /// Tracks the active effect name set via HA (for state publishing).
    pub active_effect: Arc<Mutex<Option<String>>>,
}
