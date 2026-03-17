use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::StoreError;

#[derive(Debug, Clone)]
pub struct SceneRecord {
    pub id: String,
    pub name: String,
}

#[derive(FromRow)]
pub(super) struct SceneRow {
    pub id: String,
    pub name: String,
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<SceneRecord>, StoreError> {
    let rows: Vec<SceneRow> = sqlx::query_as::<_, SceneRow>(
        "SELECT id, name FROM scenes WHERE id != ? AND name IS NOT NULL ORDER BY name",
    )
    .bind(super::active::ACTIVE_SCENE_ID)
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
        .bind(super::active::ACTIVE_SCENE_ID)
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
        .bind(super::active::ACTIVE_SCENE_ID)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    Ok(())
}
