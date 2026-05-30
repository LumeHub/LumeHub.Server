use std::collections::HashMap;

use application::SceneSnapshot;
use domain::{BlendMode, ParamDef, ParamValue, Rgb};
use effects::BuiltinEffect;
use serde::{Deserialize, Serialize};
use store::{EffectRecord, LayerRecord, SceneRecord, ZoneRecord};

// ── Device ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DeviceStatePayload {
    pub state: &'static str,
    pub brightness: u8,
    pub color_mode: &'static str,
    pub color: RgbPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

impl DeviceStatePayload {
    pub fn from_snapshot(s: &SceneSnapshot, effect: Option<&str>) -> Self {
        Self {
            state: if s.on { "ON" } else { "OFF" },
            brightness: s.brightness,
            color_mode: "rgb",
            color: s.color.into(),
            effect: effect.map(str::to_owned),
        }
    }
}

impl From<&SceneSnapshot> for DeviceStatePayload {
    fn from(s: &SceneSnapshot) -> Self {
        Self::from_snapshot(s, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RgbPayload {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Rgb> for RgbPayload {
    fn from(c: Rgb) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

impl From<RgbPayload> for Rgb {
    fn from(c: RgbPayload) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

#[derive(Deserialize)]
pub struct DeviceCommand {
    pub state: Option<String>,
    pub brightness: Option<u8>,
    pub color: Option<RgbPayload>,
    pub effect: Option<String>,
}

// ── Zones ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ZonePayload {
    pub id: String,
    pub name: String,
    pub start_pixel: u32,
    pub end_pixel: u32,
    pub transition_length: u32,
}

impl From<ZoneRecord> for ZonePayload {
    fn from(z: ZoneRecord) -> Self {
        Self {
            id: z.id,
            name: z.name,
            start_pixel: z.start_pixel,
            end_pixel: z.end_pixel,
            transition_length: z.transition_length,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateZoneCommand {
    pub name: String,
    pub start_pixel: u32,
    pub end_pixel: u32,
    #[serde(default)]
    pub transition_length: u32,
}

#[derive(Deserialize)]
pub struct UpdateZoneCommand {
    pub name: String,
    pub start_pixel: u32,
    pub end_pixel: u32,
    #[serde(default)]
    pub transition_length: u32,
}

// ── Scenes ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ScenePayload {
    pub id: String,
    pub name: String,
}

impl From<SceneRecord> for ScenePayload {
    fn from(s: SceneRecord) -> Self {
        Self {
            id: s.id,
            name: s.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateSceneCommand {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateSceneCommand {
    pub name: String,
}

// ── Active layers ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct LayerPayload {
    pub id: String,
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: BlendMode,
    pub params: HashMap<String, ParamValue>,
    pub enabled: bool,
    pub position: u32,
}

impl From<LayerRecord> for LayerPayload {
    fn from(l: LayerRecord) -> Self {
        Self {
            id: l.id,
            effect_id: l.effect_id,
            zone_id: l.zone_id,
            blend_mode: l.blend_mode,
            params: l.params,
            enabled: l.enabled,
            position: l.position,
        }
    }
}

#[derive(Deserialize)]
pub struct AddLayerCommand {
    pub effect_id: String,
    pub zone_id: String,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub params: HashMap<String, ParamValue>,
}

#[derive(Deserialize)]
pub struct UpdateLayerCommand {
    pub zone_id: Option<String>,
    pub enabled: Option<bool>,
    pub blend_mode: Option<BlendMode>,
    pub params: Option<HashMap<String, ParamValue>>,
}

#[derive(Deserialize)]
pub struct ReorderLayersCommand {
    pub ordered_ids: Vec<String>,
}

// ── Effects ──────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct EffectPayload {
    pub id: String,
    pub name: String,
    pub script: String,
    pub params: Vec<ParamDef>,
    pub builtin: bool,
}

impl From<EffectRecord> for EffectPayload {
    fn from(e: EffectRecord) -> Self {
        Self {
            id: e.id,
            name: e.name,
            script: e.script,
            params: e.params,
            builtin: false,
        }
    }
}

impl From<&BuiltinEffect> for EffectPayload {
    fn from(b: &BuiltinEffect) -> Self {
        Self {
            id: b.id(),
            name: b.name.clone(),
            script: b.script.clone(),
            params: b.params.clone(),
            builtin: true,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateEffectCommand {
    pub name: String,
    pub script: String,
    #[serde(default)]
    pub params: Vec<ParamDef>,
}

#[derive(Deserialize)]
pub struct UpdateEffectCommand {
    pub name: Option<String>,
    pub script: Option<String>,
    pub params: Option<Vec<ParamDef>>,
}
