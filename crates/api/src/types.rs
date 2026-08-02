use std::collections::HashMap;

use domain::{BlendMode, ParamValue};
use serde::Serialize;
use store::{LayerRecord, SceneRecord};

#[derive(Serialize)]
pub struct LayerResponse {
    pub id: String,
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: BlendMode,
    pub params: HashMap<String, ParamValue>,
    pub enabled: bool,
    pub position: u32,
    pub opacity: f32,
}

impl From<LayerRecord> for LayerResponse {
    fn from(r: LayerRecord) -> Self {
        Self {
            id: r.id,
            effect_id: r.effect_id,
            zone_id: r.zone_id,
            blend_mode: r.blend_mode,
            params: r.params,
            enabled: r.enabled,
            position: r.position,
            opacity: r.opacity,
        }
    }
}

#[derive(Serialize)]
pub struct SceneResponse {
    pub id: String,
    pub name: String,
}

impl From<SceneRecord> for SceneResponse {
    fn from(r: SceneRecord) -> Self {
        Self {
            id: r.id,
            name: r.name,
        }
    }
}
