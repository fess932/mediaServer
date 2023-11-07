use std::fmt::Debug;
use std::fs::read_dir;
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

    scan()
}

fn scan() {
    read_dir("/Users/fess932").expect("wtf").for_each(|x| {
        // let b = ;
        let a = x.expect("wtf");

        println!("path {} {} is file {}", a.path().to_str().expect("wtf"), a.file_name().to_str().expect("wtf"), a.metadata().expect("wtf").is_file())
    });
}