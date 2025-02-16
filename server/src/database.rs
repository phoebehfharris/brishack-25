use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use std::sync::OnceLock;

const DATABASE_URL: &str = "sqlite://sqlite.db";

pub static DB: OnceLock<Db> = OnceLock::new();

pub async fn get_db() -> &'static Db {
    DB.get().unwrap()
}

pub struct Db {
    pub pool: SqlitePool,
}

pub async fn init() {

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

    DB.get_or_init(|| Db { pool: db });

}
