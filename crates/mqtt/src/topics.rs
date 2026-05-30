pub struct Topics {
    pub prefix: String,
}

impl Topics {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    // Device
    pub fn device_state(&self) -> String {
        format!("{}/device/state", self.prefix)
    }
    pub fn device_set(&self) -> String {
        format!("{}/device/set", self.prefix)
    }

    // Zones
    pub fn zones(&self) -> String {
        format!("{}/zones", self.prefix)
    }
    pub fn zones_create(&self) -> String {
        format!("{}/zones/create", self.prefix)
    }
    pub fn zone(&self, id: &str) -> String {
        format!("{}/zones/{}", self.prefix, id)
    }
    pub fn zones_update_wildcard(&self) -> String {
        format!("{}/zones/+/update", self.prefix)
    }
    pub fn zones_delete_wildcard(&self) -> String {
        format!("{}/zones/+/delete", self.prefix)
    }

    // Scenes
    pub fn scenes(&self) -> String {
        format!("{}/scenes", self.prefix)
    }
    pub fn scenes_create(&self) -> String {
        format!("{}/scenes/create", self.prefix)
    }
    pub fn scene(&self, id: &str) -> String {
        format!("{}/scenes/{}", self.prefix, id)
    }
    pub fn scenes_update_wildcard(&self) -> String {
        format!("{}/scenes/+/update", self.prefix)
    }
    pub fn scenes_delete_wildcard(&self) -> String {
        format!("{}/scenes/+/delete", self.prefix)
    }
    pub fn scenes_load_wildcard(&self) -> String {
        format!("{}/scenes/+/load", self.prefix)
    }
    pub fn scenes_save_wildcard(&self) -> String {
        format!("{}/scenes/+/save", self.prefix)
    }

    // Active layers
    pub fn active_layers(&self) -> String {
        format!("{}/active_layers", self.prefix)
    }
    pub fn active_layers_set(&self) -> String {
        format!("{}/active_layers/set", self.prefix)
    }
    pub fn active_layers_clear(&self) -> String {
        format!("{}/active_layers/clear", self.prefix)
    }
    pub fn active_layers_add(&self) -> String {
        format!("{}/active_layers/add", self.prefix)
    }
    pub fn active_layers_reorder(&self) -> String {
        format!("{}/active_layers/reorder", self.prefix)
    }
    pub fn active_layers_update_wildcard(&self) -> String {
        format!("{}/active_layers/+/update", self.prefix)
    }
    pub fn active_layers_delete_wildcard(&self) -> String {
        format!("{}/active_layers/+/delete", self.prefix)
    }

    // Effects
    pub fn effects(&self) -> String {
        format!("{}/effects", self.prefix)
    }
    pub fn effects_create(&self) -> String {
        format!("{}/effects/create", self.prefix)
    }
    pub fn effect(&self, id: &str) -> String {
        format!("{}/effects/{}", self.prefix, id)
    }
    pub fn effects_update_wildcard(&self) -> String {
        format!("{}/effects/+/update", self.prefix)
    }
    pub fn effects_delete_wildcard(&self) -> String {
        format!("{}/effects/+/delete", self.prefix)
    }

    /// Extract the middle segment from `{prefix}/{resource}/{id}/{action}`.
    pub fn extract_id<'a>(&self, topic: &'a str, resource: &str, action: &str) -> Option<&'a str> {
        let prefix = format!("{}/{}/", self.prefix, resource);
        let suffix = format!("/{}", action);
        let middle = topic.strip_prefix(&prefix)?.strip_suffix(&suffix)?;
        if middle.contains('/') {
            return None;
        }
        Some(middle)
    }
}
