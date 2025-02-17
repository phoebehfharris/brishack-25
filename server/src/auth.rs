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
#[derive(Deserialize)]
#[derive(Clone, FromRow, Debug)]
pub struct UserRequest {
    username: String,
    password: String,
}
#[derive(ToSchema)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone, FromRow, Debug)]
pub struct UserResponse {
    id: u32,
}

/// Returns the user id if username and password correct
#[utoipa::path(
get,
path = "/api/items",
responses(
    (status = 200, description = "JSON file", body = UserResponse)
)
)]
pub async fn get_items(body: Json<UserRequest>) -> Response {
    let pool = &get_db().await.pool;
    let result = sqlx::query_as::<_, UserResponse>("SELECT id FROM users WHERE username = ? AND password = ?").bind(body.username.clone()).bind(body.password.clone()).fetch_all(pool).await.unwrap();
    Json(result).into_response()
}

/// Creates an item
#[utoipa::path(
    post,
    path = "/api/items",
    request_body = UserRequest,
    responses(
        (status = 200, description = "JSON file")
    )
    )]
    #[axum::debug_handler]
    pub async fn create_item(body: Json<UserRequest>) -> Response {
        println!("{:?}", body);
        let pool = &get_db().await.pool;
    
        sqlx::query("INSERT INTO users (username, password) VALUES (?,?)")
        .bind(body.username.clone())
        .bind(body.password.clone())
        .execute(pool)
        .await
        .unwrap();
    
        StatusCode::OK.into_response()
    }