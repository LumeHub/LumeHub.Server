use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use domain::{BlendMode, ParamValue, Rgb};

use crate::StoreError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StackLayer {
    pub effect_id: String,
    pub zone_id: String,
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub enabled: bool,
    pub params: HashMap<String, ParamValue>,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StackDevice {
    pub on: bool,
    pub brightness: u8,
    pub color: Rgb,
}

#[derive(Debug, Clone)]
pub struct StackEntry {
    pub position: i64,
    pub layers: Vec<StackLayer>,
    pub device: StackDevice,
    pub pushed_at: i64,
}

#[derive(FromRow)]
struct StackRow {
    position: i64,
    layers_json: String,
    device_json: String,
    pushed_at: i64,
}

fn entry_from_row(row: StackRow) -> Result<StackEntry, StoreError> {
    Ok(StackEntry {
        position: row.position,
        layers: serde_json::from_str(&row.layers_json)?,
        device: serde_json::from_str(&row.device_json)?,
        pushed_at: row.pushed_at,
    })
}

pub async fn push(
    pool: &SqlitePool,
    layers: &[StackLayer],
    device: &StackDevice,
) -> Result<(), StoreError> {
    let layers_json = serde_json::to_string(layers)?;
    let device_json = serde_json::to_string(device)?;
    let pushed_at = Utc::now().timestamp();

    sqlx::query("INSERT INTO state_stack (layers_json, device_json, pushed_at) VALUES (?, ?, ?)")
        .bind(layers_json)
        .bind(device_json)
        .bind(pushed_at)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn pop(pool: &SqlitePool) -> Result<Option<StackEntry>, StoreError> {
    let mut tx = pool.begin().await?;

    let row: Option<StackRow> = sqlx::query_as::<_, StackRow>(
        "SELECT position, layers_json, device_json, pushed_at
         FROM state_stack ORDER BY position DESC LIMIT 1",
    )
    .fetch_optional(&mut *tx)
    .await?;

    let Some(row) = row else {
        tx.commit().await?;
        return Ok(None);
    };

    sqlx::query("DELETE FROM state_stack WHERE position = ?")
        .bind(row.position)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Some(entry_from_row(row)?))
}

pub async fn peek(pool: &SqlitePool) -> Result<Option<StackEntry>, StoreError> {
    let row: Option<StackRow> = sqlx::query_as::<_, StackRow>(
        "SELECT position, layers_json, device_json, pushed_at
         FROM state_stack ORDER BY position DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    row.map(entry_from_row).transpose()
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<StackEntry>, StoreError> {
    let rows: Vec<StackRow> = sqlx::query_as::<_, StackRow>(
        "SELECT position, layers_json, device_json, pushed_at
         FROM state_stack ORDER BY position DESC",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(entry_from_row).collect()
}

pub async fn depth(pool: &SqlitePool) -> Result<u32, StoreError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM state_stack")
        .fetch_one(pool)
        .await?;
    Ok(count as u32)
}

pub async fn clear(pool: &SqlitePool) -> Result<(), StoreError> {
    sqlx::query("DELETE FROM state_stack").execute(pool).await?;
    Ok(())
}
