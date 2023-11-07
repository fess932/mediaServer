use sqlx::sqlite::SqlitePoolOptions;
use anyhow::Context;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://target/wtf.db")
        .await
        .context("could not connect to database url")
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    println!("Hello, world!");
}
