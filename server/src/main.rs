use std::net::SocketAddr;

use axum::Json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_axum::routes;
use utoipa_axum::router::OpenApiRouter;

use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool, FromRow};

mod example;
mod items;

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

const DATABASE_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, FromRow, Debug)]
struct Item {
    id: i64,
    description: String,
    done: bool,
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        println!("Creating sqlite db {}", DATABASE_URL);
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Database created"),
            Err(e) => panic!("{}", e),
        }
    }
    else {
        println!("db already exists");
    }

    let db = SqlitePool::connect(DATABASE_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("migrations");

    println!("{:?}", migrations);

    let m2 = sqlx::migrate::Migrator::new(migrations).await.unwrap();
    println!("{:?}", m2);
    let migration_results = m2.run(&db).await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);


    let result = sqlx::query(
        "SELECT name
        FROM sqlite_schema
        WHERE type ='table'
    AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    let result = sqlx::query("INSERT INTO items (description, done) VALUES (?,?)")
    .bind("the explanation of what this is")
    .bind(true)
    .execute(&db)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let results = sqlx::query_as::<_, Item>("SELECT * FROM items").fetch_all(&db).await.unwrap();
    for r in results {
        println!(
            "id: {}, desc: {}, done: {}",
            r.id, r.description, r.done
        );
    }
    let socket_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .routes(routes!(example::hello_lib))
    .routes(routes!(items::get_items))
    .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    axum::serve(listener, router.into_make_service())
    .await
    .unwrap()
}
