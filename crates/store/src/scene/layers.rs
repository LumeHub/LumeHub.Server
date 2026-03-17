use std::collections::HashMap;

use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use domain::{BlendMode, ParamValue};

use crate::StoreError;

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
pub(super) struct LayerRow {
    pub id: String,
    pub scene_id: String,
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: String,
    pub params: String,
    pub enabled: i64,
    pub position: i64,
}

pub struct NewLayer {
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: BlendMode,
    pub params: HashMap<String, ParamValue>,
}

pub(super) fn layer_from_row(row: LayerRow) -> Result<LayerRecord, StoreError> {
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

pub(super) fn blend_mode_to_str(mode: BlendMode) -> &'static str {
    match mode {
        BlendMode::Override => "override",
        BlendMode::Add => "add",
        BlendMode::Screen => "screen",
        BlendMode::Multiply => "multiply",
    }
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

pub async fn get_layer_in_scene(
    pool: &SqlitePool,
    id: &str,
    scene_id: &str,
) -> Result<LayerRecord, StoreError> {
    let row: Option<LayerRow> = sqlx::query_as::<_, LayerRow>(
        "SELECT id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position
         FROM scene_layers WHERE id = ? AND scene_id = ?",
    )
    .bind(id)
    .bind(scene_id)
    .fetch_optional(pool)
    .await?;
    layer_from_row(row.ok_or(StoreError::NotFound)?)
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
    scene_id: &str,
    enabled: bool,
    blend_mode: BlendMode,
    params: &HashMap<String, ParamValue>,
) -> Result<LayerRecord, StoreError> {
    let blend_str = blend_mode_to_str(blend_mode);
    let params_json = serde_json::to_string(params)?;
    let enabled_int = enabled as i64;

    let rows_affected = sqlx::query(
        "UPDATE scene_layers SET enabled = ?, blend_mode = ?, params = ? WHERE id = ? AND scene_id = ?",
    )
    .bind(enabled_int)
    .bind(blend_str)
    .bind(&params_json)
    .bind(id)
    .bind(scene_id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    get_layer(pool, id).await
}

pub async fn remove_layer(pool: &SqlitePool, id: &str, scene_id: &str) -> Result<(), StoreError> {
    let rows_affected = sqlx::query("DELETE FROM scene_layers WHERE id = ? AND scene_id = ?")
        .bind(id)
        .bind(scene_id)
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
    if ordered_ids.is_empty() {
        return Ok(());
    }

    let existing: std::collections::HashSet<String> = get_layers(pool, scene_id)
        .await?
        .into_iter()
        .map(|l| l.id)
        .collect();

    if !ordered_ids.iter().all(|id| existing.contains(id)) {
        return Err(StoreError::NotFound);
    }

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

pub(super) async fn copy_layers(
    pool: &SqlitePool,
    from_scene_id: &str,
    to_scene_id: &str,
) -> Result<(), StoreError> {
    let layers = get_layers(pool, from_scene_id).await?;
    if layers.is_empty() {
        return Ok(());
    }

    let entries = layers
        .iter()
        .map(|l| serde_json::to_string(&l.params).map(|p| (Uuid::new_v4().to_string(), l, p)))
        .collect::<Result<Vec<_>, _>>()?;

    sqlx::QueryBuilder::new(
        "INSERT INTO scene_layers (id, scene_id, effect_id, zone_id, blend_mode, params, enabled, position) ",
    )
    .push_values(&entries, |mut b, (id, layer, params)| {
        b.push_bind(id.as_str())
            .push_bind(to_scene_id)
            .push_bind(layer.effect_id.as_str())
            .push_bind(layer.zone_id.as_str())
            .push_bind(blend_mode_to_str(layer.blend_mode))
            .push_bind(params.as_str())
            .push_bind(layer.enabled as i64)
            .push_bind(layer.position as i64);
    })
    .build()
    .execute(pool)
    .await?;

    Ok(())
}
