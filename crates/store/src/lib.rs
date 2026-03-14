mod effect;
mod scene;
mod zone;

pub use effect::EffectRecord;
pub use scene::{ACTIVE_SCENE_ID, LayerRecord, SceneRecord};
pub use zone::ZoneRecord;

use std::path::Path;

use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("not found")]
    NotFound,
}

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub async fn open(path: &Path) -> Result<Self, StoreError> {
        let url = format!("sqlite://{}?mode=rwc", path.display());
        let pool = SqlitePool::connect(&url).await?;
        Self::from_pool(pool).await
    }

    pub async fn from_pool(pool: SqlitePool) -> Result<Self, StoreError> {
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Delegate CRUD methods so callers only need the Store handle.
impl Store {
    pub async fn get_zones(&self) -> Result<Vec<ZoneRecord>, StoreError> {
        zone::get_all(&self.pool).await
    }
    pub async fn get_zone(&self, id: &str) -> Result<ZoneRecord, StoreError> {
        zone::get_one(&self.pool, id).await
    }
    pub async fn create_zone(
        &self,
        name: &str,
        start_pixel: u32,
        end_pixel: u32,
        transition_length: u32,
    ) -> Result<ZoneRecord, StoreError> {
        zone::create(&self.pool, name, start_pixel, end_pixel, transition_length).await
    }
    pub async fn update_zone(
        &self,
        id: &str,
        name: &str,
        start_pixel: u32,
        end_pixel: u32,
        transition_length: u32,
    ) -> Result<ZoneRecord, StoreError> {
        zone::update(
            &self.pool,
            id,
            name,
            start_pixel,
            end_pixel,
            transition_length,
        )
        .await
    }
    pub async fn delete_zone(&self, id: &str) -> Result<(), StoreError> {
        zone::delete(&self.pool, id).await
    }

    pub async fn get_effects(&self) -> Result<Vec<EffectRecord>, StoreError> {
        effect::get_all(&self.pool).await
    }
    pub async fn get_effect(&self, id: &str) -> Result<EffectRecord, StoreError> {
        effect::get_one(&self.pool, id).await
    }
    pub async fn create_effect(
        &self,
        name: &str,
        script: &str,
        params: &[domain::ParamDef],
    ) -> Result<EffectRecord, StoreError> {
        effect::create(&self.pool, name, script, params).await
    }
    pub async fn update_effect(
        &self,
        id: &str,
        name: &str,
        script: &str,
        params: &[domain::ParamDef],
    ) -> Result<EffectRecord, StoreError> {
        effect::update(&self.pool, id, name, script, params).await
    }
    pub async fn delete_effect(&self, id: &str) -> Result<(), StoreError> {
        effect::delete(&self.pool, id).await
    }

    pub async fn get_scenes(&self) -> Result<Vec<SceneRecord>, StoreError> {
        scene::get_all(&self.pool).await
    }
    pub async fn get_scene(&self, id: &str) -> Result<SceneRecord, StoreError> {
        scene::get_one(&self.pool, id).await
    }
    pub async fn create_scene(&self, name: &str) -> Result<SceneRecord, StoreError> {
        scene::create(&self.pool, name).await
    }
    pub async fn update_scene(&self, id: &str, name: &str) -> Result<SceneRecord, StoreError> {
        scene::update(&self.pool, id, name).await
    }
    pub async fn delete_scene(&self, id: &str) -> Result<(), StoreError> {
        scene::delete(&self.pool, id).await
    }

    pub async fn get_layers(&self, scene_id: &str) -> Result<Vec<LayerRecord>, StoreError> {
        scene::get_layers(&self.pool, scene_id).await
    }
    pub async fn add_layer(
        &self,
        scene_id: &str,
        effect_id: &str,
        zone_id: &str,
        blend_mode: domain::BlendMode,
        params: &std::collections::HashMap<String, domain::ParamValue>,
    ) -> Result<LayerRecord, StoreError> {
        scene::add_layer(&self.pool, scene_id, effect_id, zone_id, blend_mode, params).await
    }
    pub async fn update_layer(
        &self,
        id: &str,
        enabled: bool,
        blend_mode: domain::BlendMode,
        params: &std::collections::HashMap<String, domain::ParamValue>,
    ) -> Result<LayerRecord, StoreError> {
        scene::update_layer(&self.pool, id, enabled, blend_mode, params).await
    }
    pub async fn remove_layer(&self, id: &str) -> Result<(), StoreError> {
        scene::remove_layer(&self.pool, id).await
    }
    pub async fn reorder_layers(
        &self,
        scene_id: &str,
        ordered_ids: &[String],
    ) -> Result<(), StoreError> {
        scene::reorder_layers(&self.pool, scene_id, ordered_ids).await
    }

    pub async fn get_active_layers(&self) -> Result<Vec<LayerRecord>, StoreError> {
        scene::get_layers(&self.pool, ACTIVE_SCENE_ID).await
    }
    pub async fn clear_active_scene(&self) -> Result<(), StoreError> {
        scene::clear_layers(&self.pool, ACTIVE_SCENE_ID).await
    }
    pub async fn load_scene_into_active(&self, scene_id: &str) -> Result<(), StoreError> {
        scene::load_into_active(&self.pool, scene_id).await
    }
    pub async fn save_active_as_scene(&self, name: &str) -> Result<SceneRecord, StoreError> {
        scene::save_active_as(&self.pool, name).await
    }
    pub async fn overwrite_scene_from_active(&self, id: &str) -> Result<(), StoreError> {
        scene::overwrite_from_active(&self.pool, id).await
    }
}
