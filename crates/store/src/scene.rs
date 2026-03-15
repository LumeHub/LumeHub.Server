use std::collections::HashMap;

use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use domain::{BlendMode, ParamValue};

use crate::StoreError;

pub const ACTIVE_SCENE_ID: &str = "__active__";

#[derive(Debug, Clone)]
pub struct SceneRecord {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LayerRecord {
    pub id: String,
    pub scene_id: String,
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: BlendMode,
    pub params: HashMap<String, ParamValue>,
    pub enabled: bool,
    pub position: u32,
}

#[derive(FromRow)]
struct SceneRow {
    id: String,
    name: String,
}

#[derive(FromRow)]
struct LayerRow {
    id: String,
    scene_id: String,
    effect_id: String,
    zone_id: String,
    blend_mode: String,
    params: String,
    enabled: i64,
    position: i64,
}

fn layer_from_row(row: LayerRow) -> Result<LayerRecord, StoreError> {
    Ok(LayerRecord {
        blend_mode: BlendMode::from(row.blend_mode.as_str()),
        params: serde_json::from_str(&row.params)?,
        enabled: row.enabled != 0,
        position: row.position as u32,
        id: row.id,
        scene_id: row.scene_id,
        effect_id: row.effect_id,
        zone_id: row.zone_id,
    })
}

fn blend_mode_to_str(mode: BlendMode) -> &'static str {
    match mode {
        BlendMode::Override => "override",
        BlendMode::Add => "add",
        BlendMode::Screen => "screen",
        BlendMode::Multiply => "multiply",
    }
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<SceneRecord>, StoreError> {
    let rows: Vec<SceneRow> = sqlx::query_as::<_, SceneRow>(
        "SELECT id, name FROM scenes WHERE id != ? AND name IS NOT NULL ORDER BY name",
    )
    .bind(ACTIVE_SCENE_ID)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| SceneRecord {
            id: r.id,
            name: r.name,
        })
        .collect())
}

pub async fn get_one(pool: &SqlitePool, id: &str) -> Result<SceneRecord, StoreError> {
    let row: Option<SceneRow> =
        sqlx::query_as::<_, SceneRow>("SELECT id, name FROM scenes WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    row.map(|r| SceneRecord {
        id: r.id,
        name: r.name,
    })
    .ok_or(StoreError::NotFound)
}

pub async fn create(pool: &SqlitePool, name: &str) -> Result<SceneRecord, StoreError> {
    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO scenes (id, name) VALUES (?, ?)")
        .bind(&id)
        .bind(name)
        .execute(pool)
        .await?;
    get_one(pool, &id).await
}

pub async fn update(pool: &SqlitePool, id: &str, name: &str) -> Result<SceneRecord, StoreError> {
    let rows_affected = sqlx::query("UPDATE scenes SET name = ? WHERE id = ? AND id != ?")
        .bind(name)
        .bind(id)
        .bind(ACTIVE_SCENE_ID)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    get_one(pool, id).await
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), StoreError> {
    let rows_affected = sqlx::query("DELETE FROM scenes WHERE id = ? AND id != ?")
        .bind(id)
        .bind(ACTIVE_SCENE_ID)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    Ok(())
}

pub async fn get_layers(pool: &SqlitePool, scene_id: &str) -> Result<Vec<LayerRecord>, StoreError> {
    let rows: Vec<LayerRow> = sqlx::query_as::<_, LayerRow>(
        "SELECT id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position
         FROM scene_layers WHERE scene_id = ? ORDER BY position",
    )
    .bind(scene_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(layer_from_row).collect()
}

pub async fn add_layer(
    pool: &SqlitePool,
    scene_id: &str,
    effect_id: &str,
    zone_id: &str,
    blend_mode: BlendMode,
    params: &HashMap<String, ParamValue>,
) -> Result<LayerRecord, StoreError> {
    let id = Uuid::new_v4().to_string();
    let blend_str = blend_mode_to_str(blend_mode);
    let params_json = serde_json::to_string(params)?;

    let position: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(position), -1) + 1 FROM scene_layers WHERE scene_id = ?",
    )
    .bind(scene_id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "INSERT INTO scene_layers (id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position)
         VALUES (?, ?, ?, ?, ?, ?, 1, ?)",
    )
    .bind(&id)
    .bind(scene_id)
    .bind(effect_id)
    .bind(zone_id)
    .bind(blend_str)
    .bind(&params_json)
    .bind(position)
    .execute(pool)
    .await?;

    get_layer(pool, &id).await
}

pub async fn update_layer(
    pool: &SqlitePool,
    id: &str,
    enabled: bool,
    blend_mode: BlendMode,
    params: &HashMap<String, ParamValue>,
) -> Result<LayerRecord, StoreError> {
    let blend_str = blend_mode_to_str(blend_mode);
    let params_json = serde_json::to_string(params)?;
    let enabled_int = enabled as i64;

    let rows_affected =
        sqlx::query("UPDATE scene_layers SET enabled = ?, blend_mode = ?, params = ? WHERE id = ?")
            .bind(enabled_int)
            .bind(blend_str)
            .bind(&params_json)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    get_layer(pool, id).await
}

pub async fn remove_layer(pool: &SqlitePool, id: &str) -> Result<(), StoreError> {
    let rows_affected = sqlx::query("DELETE FROM scene_layers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    Ok(())
}

pub async fn reorder_layers(
    pool: &SqlitePool,
    scene_id: &str,
    ordered_ids: &[String],
) -> Result<(), StoreError> {
    let mut tx = pool.begin().await?;
    for (position, id) in ordered_ids.iter().enumerate() {
        sqlx::query("UPDATE scene_layers SET position = ? WHERE id = ? AND scene_id = ?")
            .bind(position as i64)
            .bind(id)
            .bind(scene_id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn clear_layers(pool: &SqlitePool, scene_id: &str) -> Result<(), StoreError> {
    sqlx::query("DELETE FROM scene_layers WHERE scene_id = ?")
        .bind(scene_id)
        .execute(pool)
        .await?;
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

async fn copy_layers(
    pool: &SqlitePool,
    from_scene_id: &str,
    to_scene_id: &str,
) -> Result<(), StoreError> {
    let layers = get_layers(pool, from_scene_id).await?;
    for layer in layers {
        let params_json = serde_json::to_string(&layer.params)?;
        let new_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO scene_layers (id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&new_id)
        .bind(to_scene_id)
        .bind(&layer.effect_id)
        .bind(&layer.zone_id)
        .bind(blend_mode_to_str(layer.blend_mode))
        .bind(&params_json)
        .bind(layer.enabled as i64)
        .bind(layer.position as i64)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_layer(pool: &SqlitePool, id: &str) -> Result<LayerRecord, StoreError> {
    let row: Option<LayerRow> = sqlx::query_as::<_, LayerRow>(
        "SELECT id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position
         FROM scene_layers WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    layer_from_row(row.ok_or(StoreError::NotFound)?)
}
