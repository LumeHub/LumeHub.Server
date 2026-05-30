mod client;
mod discovery;
mod publish;
mod types;

pub mod config;
pub mod ctx;
pub mod handlers;
pub mod topics;

pub use config::MqttConfig;

use std::sync::Arc;

use application::{SceneRuntime, StateEventBus};
use effects::BuiltinEffect;
use store::Store;

pub fn spawn(
    config: MqttConfig,
    runtime: Arc<dyn SceneRuntime>,
    store: Arc<Store>,
    event_bus: StateEventBus,
    builtins: Vec<BuiltinEffect>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(client::run(config, runtime, store, event_bus, builtins))
}
