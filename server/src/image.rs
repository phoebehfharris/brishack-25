use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use tokio_util::io::ReaderStream;
use axum::body::Body;

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
