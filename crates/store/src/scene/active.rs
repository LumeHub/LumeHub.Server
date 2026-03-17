use sqlx::SqlitePool;
use uuid::Uuid;

use crate::StoreError;

use super::layers::{NewLayer, blend_mode_to_str, clear_layers, copy_layers};
use super::scenes::{SceneRecord, create, get_one};

pub const ACTIVE_SCENE_ID: &str = "__active__";

pub async fn replace_active_layers(
    pool: &SqlitePool,
    layers: &[NewLayer],
) -> Result<(), StoreError> {
    let entries = layers
        .iter()
        .enumerate()
        .map(|(pos, l)| {
            serde_json::to_string(&l.params)
                .map(|params| (Uuid::new_v4().to_string(), l, params, pos as i64))
                .map_err(StoreError::from)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM scene_layers WHERE scene_id = ?")
        .bind(ACTIVE_SCENE_ID)
        .execute(&mut *tx)
        .await?;

    if !entries.is_empty() {
        sqlx::QueryBuilder::new(
            "INSERT INTO scene_layers (id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position) ",
        )
        .push_values(&entries, |mut b, (id, layer, params, pos)| {
            b.push_bind(id.as_str())
                .push_bind(ACTIVE_SCENE_ID)
                .push_bind(layer.effect_id.as_str())
                .push_bind(layer.zone_id.as_str())
                .push_bind(blend_mode_to_str(layer.blend_mode))
                .push_bind(params.as_str())
                .push_bind(1i64)
                .push_bind(*pos);
        })
        .build()
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn load_into_active(pool: &SqlitePool, scene_id: &str) -> Result<(), StoreError> {
    get_one(pool, scene_id).await?;
    clear_layers(pool, ACTIVE_SCENE_ID).await?;
    copy_layers(pool, scene_id, ACTIVE_SCENE_ID).await
}

pub async fn save_active_as(pool: &SqlitePool, name: &str) -> Result<SceneRecord, StoreError> {
    let scene = create(pool, name).await?;
    copy_layers(pool, ACTIVE_SCENE_ID, &scene.id).await?;
    Ok(scene)
}

pub async fn overwrite_from_active(pool: &SqlitePool, id: &str) -> Result<(), StoreError> {
    if id == ACTIVE_SCENE_ID {
        return Err(StoreError::NotFound);
    }
    clear_layers(pool, id).await?;
    copy_layers(pool, ACTIVE_SCENE_ID, id).await?;
    Ok(())
}
