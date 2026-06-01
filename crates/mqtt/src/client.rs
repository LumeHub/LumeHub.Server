use std::sync::{Arc, Mutex};
use std::time::Duration;

use application::{SceneRuntime, SceneSnapshot, StateEventBus};
use effects::BuiltinEffect;
use rumqttc::{AsyncClient, ConnectReturnCode, Event, MqttOptions, Packet, QoS};
use store::Store;
use tokio::sync::broadcast::error::RecvError;

use crate::config::MqttConfig;
use crate::ctx::Ctx;
use crate::discovery::publish_ha_discovery;
use crate::handlers::dispatch;
use crate::publish::publish_all_initial;
use crate::topics::Topics;

pub async fn run(
    config: MqttConfig,
    runtime: Arc<dyn SceneRuntime>,
    store: Arc<Store>,
    event_bus: StateEventBus,
    builtins: Vec<BuiltinEffect>,
) {
    let builtins = Arc::new(builtins);

    // Forward bus snapshots to a channel so we can select! on them.
    let (snap_tx, mut snap_rx) = tokio::sync::mpsc::unbounded_channel::<SceneSnapshot>();
    {
        let snap_tx = snap_tx.clone();
        let mut bus_rx = event_bus.subscribe();
        tokio::spawn(async move {
            loop {
                match bus_rx.recv().await {
                    Ok(snapshot) => {
                        let _ = snap_tx.send(snapshot);
                    }
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => break,
                }
            }
        });
    }

    let client_id = format!("lumehub-{}", &config.device_id);

    loop {
        let mut opts = MqttOptions::new(&client_id, &config.host, config.port);
        opts.set_keep_alive(Duration::from_secs(60));
        opts.set_clean_session(true);
        opts.set_max_packet_size(1024 * 1024, 1024 * 1024);
        let username = config.username.as_deref().filter(|s| !s.is_empty());
        let password = config.password.as_deref().filter(|s| !s.is_empty());
        if let (Some(u), Some(p)) = (username, password) {
            opts.set_credentials(u, p);
        }

        // Large capacity so publish_all_initial never blocks the event loop.
        let (client, mut eventloop) = AsyncClient::new(opts, 256);

        let ctx = Ctx {
            client: client.clone(),
            topics: Topics::new(&config.topic_prefix),
            config: config.clone(),
            runtime: Arc::clone(&runtime),
            store: Arc::clone(&store),
            builtins: Arc::clone(&builtins),
            active_effect: Arc::new(Mutex::new(None)),
        };

        let mut connected = false;

        loop {
            tokio::select! {
                event = eventloop.poll() => {
                    match event {
                        Ok(Event::Incoming(Packet::ConnAck(ack))) => {
                            if ack.code != ConnectReturnCode::Success {
                                eprintln!("[mqtt] broker rejected connection: {:?}", ack.code);
                                break;
                            }
                            connected = true;
                            println!("[mqtt] connected to {}:{}", config.host, config.port);

                            // Subscribe to all command topics.
                            subscribe_all(&ctx.client, &ctx.topics).await;

                            // Spawn initial publish so the event loop keeps polling
                            // while retained state is pushed to the broker.
                            let pub_client   = ctx.client.clone();
                            let pub_topics   = Topics::new(&ctx.topics.prefix);
                            let pub_runtime  = Arc::clone(&ctx.runtime);
                            let pub_store    = Arc::clone(&ctx.store);
                            let pub_builtins = Arc::clone(&ctx.builtins);
                            let pub_config   = ctx.config.clone();
                            let pub_builtins2 = Arc::clone(&ctx.builtins);
                            tokio::spawn(async move {
                                println!("[mqtt] publishing retained state...");
                                publish_all_initial(&pub_client, &pub_topics, &pub_runtime, &pub_store, &pub_builtins).await;
                                if pub_config.ha_discovery {
                                    println!("[mqtt] publishing HA discovery...");
                                    publish_ha_discovery(&pub_client, &pub_config, &pub_store, &pub_builtins2).await;
                                    println!("[mqtt] HA discovery published");
                                }
                            });
                        }
                        Ok(Event::Incoming(Packet::Publish(p))) => {
                            if connected {
                                dispatch(&ctx, p).await;
                            }
                        }
                        Err(e) => {
                            if connected {
                                eprintln!("[mqtt] disconnected: {e}");
                            } else {
                                eprintln!("[mqtt] connection failed: {e}");
                            }
                            break;
                        }
                        _ => {}
                    }
                }
                Some(snapshot) = snap_rx.recv() => {
                    if connected {
                        let effect = ctx.active_effect.lock().unwrap().clone();
                        crate::publish::publish_device_state(&ctx.client, &ctx.topics, &snapshot, effect.as_deref()).await;
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn subscribe_all(client: &AsyncClient, topics: &Topics) {
    let subs = [
        topics.device_set(),
        topics.zones_create(),
        topics.zones_update_wildcard(),
        topics.zones_delete_wildcard(),
        topics.scenes_create(),
        topics.scenes_update_wildcard(),
        topics.scenes_delete_wildcard(),
        topics.scenes_load_wildcard(),
        topics.scenes_save_wildcard(),
        topics.active_layers_set(),
        topics.active_layers_clear(),
        topics.active_layers_add(),
        topics.active_layers_reorder(),
        topics.active_layers_update_wildcard(),
        topics.active_layers_delete_wildcard(),
        topics.effects_create(),
        topics.effects_update_wildcard(),
        topics.effects_delete_wildcard(),
        format!("{}/active_scene/set", topics.prefix),
    ];

    for topic in &subs {
        if let Err(e) = client.subscribe(topic, QoS::AtLeastOnce).await {
            eprintln!("[mqtt] subscribe error for '{topic}': {e}");
        }
    }
}
