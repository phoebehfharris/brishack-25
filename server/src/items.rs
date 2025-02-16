use axum::{extract::FromRequestParts, http::StatusCode};
use axum::response::Response;
// use axum::Json;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;
use sqlx::FromRow;
use axum::extract::{Path, Json};

use crate::database::get_db;

#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Deserialize)]
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

/// Creates an item
#[utoipa::path(
post,
path = "/api/items",
request_body = Item,
responses(
    (status = 200, description = "JSON file")
)
)]
#[axum::debug_handler]
pub async fn create_item(body: Json<Item>) -> Response {
    println!("{:?}", body);
    let pool = &get_db().await.pool;

    sqlx::query("INSERT INTO items (name, description, damages) VALUES (?,?,?)")
    .bind(body.name.clone())
    .bind(body.description.clone())
    .bind(body.damages.clone())
    .execute(pool)
    .await
    .unwrap();

    StatusCode::OK.into_response()
}

/// Get item by id
///
/// Get item from database by item id
#[utoipa::path(
get,
path = "/api/items/{id}",
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

    match result2 {
        Ok(item) => Json(item).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
