use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;
use serde::Serialize;
use axum::response::IntoResponse;

#[derive(ToSchema)]
#[derive(Serialize)]
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
    (status = 200, description = "JSON file", body = Item)
)
)]
pub async fn get_items() -> Response {
    Json(Item {
        id: 25,
         name: String::from("Leafblower"),
         description: String::from("it blows leaf"),
         damages: String::from("it's not broken"),
    }).into_response()
}
