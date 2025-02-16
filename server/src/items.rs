use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;
use serde::Serialize;
use axum::response::IntoResponse;
use sqlx::FromRow;
use axum::extract::Path;

use crate::database::get_db;

#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
pub struct Item {
    id: u32,
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

/// Get item by id
///
/// Get item from database by item id
#[utoipa::path(
get,
path = "/items/{id}",
responses(
    (status = 200, description = "Item found successfully", body = Item),
    (status = NOT_FOUND, description = "Item was not found", body = ())
),
params(
    ("id", description = "Item database id to get Item for"),
)
)]
pub async fn get_item_by_id(Path(id): Path<u32>) -> Response  {
    let pool = &get_db().await.pool;
    println!("the item id {}",id);
    let result2 = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id=?").bind(id).fetch_one(pool).await;

    if result2.is_ok() {
        return Json(result2.unwrap()).into_response();
    }
    else {
        return Json(()).into_response();
    }
}
