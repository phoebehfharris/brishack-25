use std::ptr::null;

use axum::response::Response;
use axum::Json;
use utoipa::openapi::request_body;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;
use sqlx::FromRow;
use axum::debug_handler;

use crate::database::get_db;

#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
struct Tag {
    id: u32,
    name: String,
    generic: bool
}
#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
#[derive(Deserialize)]
pub struct GetTagsRequest {
    with: Option<String>,
    generic: Option<bool>
}

/// Returns the tags
#[utoipa::path(
get,
path = "/api/tags",
request_body = GetTagsRequest,
responses(
    (status = 200, description = "JSON file", body = [Tag])
)
)]
#[axum::debug_handler]
pub async fn get_tags(body: Json<GetTagsRequest>) -> Response {
    let pool = &get_db().await.pool;
    let mut results;
    if body.clone().with.is_some() && body.clone().generic.is_some() {
        println!("here 1");
        results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name LIKE ? AND generic LIKE ?").bind(body.clone().with.clone().unwrap()).bind(body.generic.unwrap()).fetch_all(pool).await.unwrap();

    } else if body.with.is_some() {
        println!("here 2");
        println!("{:?}", body.with);
        results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name LIKE ?").bind(format!("%{}%",body.clone().with.clone().unwrap())).fetch_all(pool).await.unwrap();

    } else if body.generic.is_some() {
        println!("here 3");
        results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE generic LIKE ?").bind(body.generic.unwrap()).fetch_all(pool).await.unwrap();

    } else {
        println!("here 4");
        results = sqlx::query_as::<_, Tag>("SELECT * FROM tags").fetch_all(pool).await.unwrap();
    }
   
    
    Json(results).into_response()
}
