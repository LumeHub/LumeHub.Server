use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::StoreError;

#[derive(Debug, Clone)]
pub struct ZoneRecord {
    pub id: String,
    pub name: String,
    pub start_pixel: u32,
    pub end_pixel: u32,
    pub transition_length: u32,
}

#[derive(FromRow)]
struct ZoneRow {
    id: String,
    name: String,
    start_pixel: i64,
    end_pixel: i64,
    transition_length: i64,
}

impl From<ZoneRow> for ZoneRecord {
    fn from(r: ZoneRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            start_pixel: r.start_pixel as u32,
            end_pixel: r.end_pixel as u32,
            transition_length: r.transition_length as u32,
        }
    }
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<ZoneRecord>, StoreError> {
    let rows: Vec<ZoneRow> = sqlx::query_as::<_, ZoneRow>(
        "SELECT id, name, start_pixel, end_pixel, transition_length FROM zones ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(ZoneRecord::from).collect())
}

pub async fn get_one(pool: &SqlitePool, id: &str) -> Result<ZoneRecord, StoreError> {
    sqlx::query_as::<_, ZoneRow>(
        "SELECT id, name, start_pixel, end_pixel, transition_length FROM zones WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .map(ZoneRecord::from)
    .ok_or(StoreError::NotFound)
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    start_pixel: u32,
    end_pixel: u32,
    transition_length: u32,
) -> Result<ZoneRecord, StoreError> {
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO zones (id, name, start_pixel, end_pixel, transition_length)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(start_pixel)
    .bind(end_pixel)
    .bind(transition_length)
    .execute(pool)
    .await?;
    get_one(pool, &id).await
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    start_pixel: u32,
    end_pixel: u32,
    transition_length: u32,
) -> Result<ZoneRecord, StoreError> {
    let rows_affected = sqlx::query(
        "UPDATE zones SET name = ?, start_pixel = ?, end_pixel = ?, transition_length = ?
         WHERE id = ?",
    )
    .bind(name)
    .bind(start_pixel)
    .bind(end_pixel)
    .bind(transition_length)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    get_one(pool, id).await
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), StoreError> {
    let rows_affected = sqlx::query("DELETE FROM zones WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    Ok(())
}
