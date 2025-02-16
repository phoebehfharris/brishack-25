use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;
use serde::Serialize;
use axum::response::IntoResponse;

#[derive(ToSchema)]
#[derive(Serialize)]
struct Pet {
    id: u64,
    name: String,
}

/// Return JSON version of an hello world message
#[utoipa::path(
get,
path = "/api/hello.json",
responses(
    (status = 200, description = "JSON file", body = Pet)
)
)]
pub async fn hello_lib() -> Response {
    Json(Pet {
        id: 25,
         name: String::from("George"),
    }).into_response()
}
