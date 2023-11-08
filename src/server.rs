use axum::body::{boxed, Body, BoxBody};
use axum::http::{Request, StatusCode, Uri};
use axum::response::Response;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct File {
    id: i64,
    name: String,
    path: String,
}

// basic handler that responds with a static string
pub async fn root(Extension(pool): Extension<SqlitePool>) -> Json<Vec<File>> {
    let a = sqlx::query_as::<_, File>("select * from files")
        .fetch_all(&pool)
        .await
        .expect("wtf");

    Json(a)
}

pub async fn file_handler(
    dir: Extension<String>,
    uri: Uri,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    println!("uri {}", uri.clone());
    let res = get_static_file(dir.clone(), uri.clone()).await?;
    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(dir, uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(
    dir: Extension<String>,
    uri: Uri,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new(dir.as_str()).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
