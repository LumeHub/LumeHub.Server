use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use domain::ParamDef;

use crate::StoreError;

#[derive(Debug, Clone)]
pub struct EffectRecord {
    pub id: String,
    pub name: String,
    pub script: String,
    pub params: Vec<ParamDef>,
}

#[derive(FromRow)]
struct EffectRow {
    id: String,
    name: String,
    script: String,
    params: String,
}

fn from_row(row: EffectRow) -> Result<EffectRecord, StoreError> {
    Ok(EffectRecord {
        params: serde_json::from_str(&row.params)?,
        id: row.id,
        name: row.name,
        script: row.script,
    })
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<EffectRecord>, StoreError> {
    let rows: Vec<EffectRow> = sqlx::query_as::<_, EffectRow>(
        "SELECT id, name, script, params FROM effects ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(from_row).collect()
}

pub async fn get_one(pool: &SqlitePool, id: &str) -> Result<EffectRecord, StoreError> {
    let row: Option<EffectRow> =
        sqlx::query_as::<_, EffectRow>("SELECT id, name, script, params FROM effects WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    from_row(row.ok_or(StoreError::NotFound)?)
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    script: &str,
    params: &[ParamDef],
) -> Result<EffectRecord, StoreError> {
    let id = Uuid::new_v4().to_string();
    let params_json = serde_json::to_string(params)?;
    sqlx::query("INSERT INTO effects (id, name, script, params) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(name)
        .bind(script)
        .bind(&params_json)
        .execute(pool)
        .await?;
    get_one(pool, &id).await
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    script: &str,
    params: &[ParamDef],
) -> Result<EffectRecord, StoreError> {
    let params_json = serde_json::to_string(params)?;
    let rows_affected =
        sqlx::query("UPDATE effects SET name = ?, script = ?, params = ? WHERE id = ?")
            .bind(name)
            .bind(script)
            .bind(&params_json)
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
    let rows_affected = sqlx::query("DELETE FROM effects WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StoreError::NotFound);
    }
    Ok(())
}
