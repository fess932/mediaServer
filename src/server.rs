use axum::body::Body;
use axum::http::{Request, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct File {
    name: String,
    path: String,
    parent_path: String,

    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    _type: String,
}

// basic handler that responds with a static string
pub async fn root(Extension(pool): Extension<SqlitePool>) -> Json<Vec<File>> {
    let a = sqlx::query_as::<_, File>("select * from files")
        .fetch_all(&pool)
        .await
        .expect("wtf");

    Json(a)
}

pub async fn dirs(Extension(pool): Extension<SqlitePool>) -> Json<Vec<File>> {
    // language=sqlite
    let query = "select * from files where type=='dir'";
    let a = sqlx::query_as::<_, File>(query)
        .fetch_all(&pool)
        .await
        .expect("wtf");

    Json(a)
}

pub async fn file_handler(
    dir: Extension<String>,
    uri: Uri,
) -> Result<Response<Body>, (StatusCode, String)> {
    println!("origin uri {}", uri.clone());
    let uri = uri.to_string();

    let non_stripped_uri = Path::new(uri.as_str());
    let strip_prefix = Path::new(dir.as_str());

    println!(
        "l {} p {}",
        non_stripped_uri.to_str().expect("wtf"),
        strip_prefix.to_str().expect("wtf")
    );

    let new_uri = format!(
        "/{}",
        non_stripped_uri
            .strip_prefix(strip_prefix)
            .expect("wtf")
            .to_str()
            .expect("wtf")
    );

    println!("stripped {}", new_uri);
    let new_uri = Uri::from_str(new_uri.as_str()).expect("wtf");
    println!("uri {}", new_uri.clone());

    let res = get_static_file(dir.clone(), new_uri.clone()).await?;
    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", new_uri).parse() {
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
) -> Result<Response<axum::body::Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new(dir.as_str()).oneshot(req).await {
        Ok(res) => Ok(Body::new(res).into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
