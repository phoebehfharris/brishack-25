use std::net::SocketAddr;

use axum::{routing::get, Json};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::response::{IntoResponse, Response};
use utoipa_axum::routes;
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
get,
path = "/api-docs/openapi.json",
responses(
    (status = 200, description = "JSON file", body = ())
)
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// Return JSON version of an hello world message
#[utoipa::path(
get,
path = "/api/hello.json",
responses(
    (status = 200, description = "JSON file", body = ())
)
)]
async fn hello_lib() -> Response {
    "hello from the library".into_response()
}

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi()).routes(routes!(hello_lib)).split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    axum::serve(listener, router.into_make_service())
    .await
    .unwrap()
}
