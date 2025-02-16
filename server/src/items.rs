use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;
use serde::Serialize;
use axum::response::IntoResponse;
use sqlx::FromRow;

use crate::database::get_db;

#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
struct Item {
    id: u64,
    name: String,
    description: String,
    damages: String,
}

/// Returns the items
#[utoipa::path(
get,
path = "/api/items",
responses(
    (status = 200, description = "JSON file", body = [Item])
)
)]
pub async fn get_items() -> Response {
    let pool = &get_db().await.pool;
    let results = sqlx::query_as::<_, Item>("SELECT * FROM items").fetch_all(pool).await.unwrap();
    Json(results).into_response()
}
