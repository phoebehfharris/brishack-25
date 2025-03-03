use std::net::SocketAddr;

use axum::Json;
use database::Db;
use image::{create_image, get_image};
use items::create_item;
use tags::get_tags;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_axum::routes;
use utoipa_axum::router::OpenApiRouter;

use sqlx::{FromRow, Row};

mod example;
mod items;
mod database;
mod tags;
mod auth;
mod image;

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

#[derive(Clone, FromRow, Debug)]
struct Item {
    id: i64,
    description: String,
    done: bool,
}

#[tokio::main]
async fn main() {

    database::init().await;
    let pool = &database::get_db().await.pool;

    let result = sqlx::query(
        "SELECT name
        FROM sqlite_schema
        WHERE type ='table'
    AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(pool)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    let result = sqlx::query("INSERT INTO items (description, name, damages) VALUES (?,?,?)")
    .bind("the explanation of what this is")
    .bind("leafblower")
    .bind("this is not damaged")
    .execute(pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let result = sqlx::query("INSERT INTO tags (name, generic) VALUES (?,?)")
    .bind("lawnmower")
    .bind(true)
    .execute(pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let result = sqlx::query("INSERT INTO tags (name, generic) VALUES (?,?)")
    .bind("washing machine")
    .bind(true)
    .execute(pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let result = sqlx::query("INSERT INTO tags (name, generic) VALUES (?,?)")
    .bind("the kidney i forgot in the fridge last week")
    .bind(false)
    .execute(pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let result = sqlx::query("INSERT INTO tags (name, generic) VALUES (?,?)")
    .bind("my left kidney")
    .bind(false)
    .execute(pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let socket_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .routes(routes!(example::hello_lib))
    .routes(routes!(items::get_items))
    .routes(routes!(items::get_item_by_id))
    .routes(routes!(items::create_item))
    .routes(routes!(items::delete_item_by_id))
    .routes(routes!(tags::get_tags))
    .routes(routes!(tags::create_tag))
    .routes(routes!(tags::get_items))
    .routes(routes!(image::get_image))
    .routes(routes!(image::create_image))
    .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    axum::serve(listener, router.into_make_service())
    .await
    .unwrap()
}
