use std::ptr::null;

use axum::{http::StatusCode, response::Response};
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
pub struct Tag {
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

#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
#[derive(Deserialize)]
pub struct PostTagRequest {
    name: String,
    generic: bool
}
/// Creates a tag
#[utoipa::path(
    post,
    path = "/api/tags",
    request_body = PostTagRequest,
    responses(
        (status = 200, description = "JSON file")
    )
    )]
    #[axum::debug_handler]
    pub async fn create_tag(body: Json<PostTagRequest>) -> Response {
        println!("{:?}", body);
        let pool = &get_db().await.pool;
    
        sqlx::query("INSERT INTO tags (name, generic) VALUES (?,?)")
        .bind(body.name.clone())
        .bind(body.generic.clone())
        .execute(pool)
        .await
        .unwrap();
    
        StatusCode::OK.into_response()
    }

    /// TODO does not currently allow multiple tags
#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Clone, FromRow, Debug)]
#[derive(Deserialize)]
pub struct ListTagRequest {
    tags: String,
    available: bool
}
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

/// Returns list of relevant tagged items
#[utoipa::path(
    get,
    path = "/api/tags/list",
    request_body = GetTagsRequest,
    responses(
        (status = 200, description = "JSON file", body = [ListTagRequest]),
        (status = NOT_FOUND, description = "Item was not found", body = ())

    )
    )]
    #[axum::debug_handler]
    pub async fn get_items(body: Json<ListTagRequest>) -> Response {
        let pool = &get_db().await.pool;
        println!("the tag {}",body.tags);
        let items = sqlx::query_as::<_, Item>("SELECT * FROM items i JOIN item_tags it ON i.id = it.item_id JOIN tags t ON it.tag_id = t.id WHERE t.name = ? AND i.available = ?").bind(body.tags.clone()).bind(body.available).fetch_all(pool).await;

    match items {
        Ok(items) => Json(items).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
    }

//     /// Returns the tags
// #[utoipa::path(
//     get,
//     path = "/api/tags",
//     request_body = GetTagsRequest,
//     responses(
//         (status = 200, description = "JSON file", body = [Tag])
//     )
//     )]
//     #[axum::debug_handler]
//     pub async fn get_tags(body: Json<GetTagsRequest>) -> Response {
//         let pool = &get_db().await.pool;
//         let mut results;
//         if body.clone().with.is_some() && body.clone().generic.is_some() {
//             println!("here 1");
//             results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name LIKE ? AND generic LIKE ?").bind(body.clone().with.clone().unwrap()).bind(body.generic.unwrap()).fetch_all(pool).await.unwrap();
    
//         } else if body.with.is_some() {
//             println!("here 2");
//             println!("{:?}", body.with);
//             results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name LIKE ?").bind(format!("%{}%",body.clone().with.clone().unwrap())).fetch_all(pool).await.unwrap();
    
//         } else if body.generic.is_some() {
//             println!("here 3");
//             results = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE generic LIKE ?").bind(body.generic.unwrap()).fetch_all(pool).await.unwrap();
    
//         } else {
//             println!("here 4");
//             results = sqlx::query_as::<_, Tag>("SELECT * FROM tags").fetch_all(pool).await.unwrap();
//         }
       
        
//         Json(results).into_response()
//     }

