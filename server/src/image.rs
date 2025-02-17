use axum::extract::Path;
use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use tokio_util::io::ReaderStream;
use axum::body::Body;

use std::fs::File;
use std::io::prelude::*;

/// Get image by id
///
/// Get image from database by image id
#[utoipa::path(
get,
path = "/api/images/{id}",
responses(
    (status = 200, description = "Image found successfully", body = Vec<u8>, content_type = "image/png"),
          (status = NOT_FOUND, description = "Image was not found", body = ())
),
params(
    ("id", description = "Image database id to get Item for"),
)
)]
pub async fn get_image(Path(id): Path<u32>) -> Response {

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut filepath = std::path::PathBuf::from(&crate_dir);
    filepath.push("images");
    filepath.push(format!("{}.png", id.to_string()));

    let file = match tokio::fs::File::open(&filepath).await {
        Ok(file) => file,
        Err(err) => {return (StatusCode::NOT_FOUND, format!("File not found: {}", err)).into_response()}
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    body.into_response()
}

/// Creates an image
#[utoipa::path(
post,
path = "/api/images",
request_body(
    content(
             ("image/png"),
    ),
),
responses(
    (status = 200, description = "JSON file")
)
)]
#[axum::debug_handler]
pub async fn create_image(body: Json<String>) -> Response {
    // println!("{:?}", body.0.as_bytes().hex);

    let decoded = hex::decode(body.0).unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut filepath = std::path::PathBuf::from(&crate_dir);
    filepath.push("images");
    filepath.push("id.png");

    let mut file = File::create(filepath).unwrap();

    file.write_all(decoded.as_slice()).unwrap();
    // let pool = &get_db().await.pool;
    //
    // sqlx::query("INSERT INTO items (name, description, damages) VALUES (?,?,?)")
    // .bind(body.name.clone())
    // .bind(body.description.clone())
    // .bind(body.damages.clone())
    // .execute(pool)
    // .await
    // .unwrap();

    StatusCode::OK.into_response()
}
