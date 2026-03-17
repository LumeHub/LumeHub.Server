use domain::ParamValue;
use sqlx::sqlite::SqlitePoolOptions;
use store::Store;

pub async fn in_memory_store() -> Store {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    Store::from_pool(pool).await.unwrap()
}

pub fn make_params() -> std::collections::HashMap<String, ParamValue> {
    [("speed".to_string(), ParamValue::Number(2.0))].into()
}
