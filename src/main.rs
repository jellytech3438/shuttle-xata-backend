pub mod user;

use user::*;

use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
    Extension, Json, Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_runtime::SecretStore;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize)]
pub struct Post {
    time: DateTime<Local>,
    categories: Vec<String>,
    title: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    db: MySqlPool,
}

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses((status = 200, description = "JSON file", body = ()))
    )]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

// handle for /
async fn root() -> Json<Post> {
    Json(Post {
        time: Local::now(),
        categories: vec!["test post".to_owned()],
        title: "root title".to_owned(),
    })
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let user = secrets.get("DB_USER").unwrap();
    let password = secrets.get("DB_PASW").unwrap();
    let host = secrets.get("DB_HOST").unwrap();
    let name = secrets.get("DB_NAME").unwrap();
    let db_url = format!("mysql://{user}:{password}@{host}/{name}");
    println!("{}", db_url);
    let mut pool = MySqlPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("Failed connect to database");
    let router = Router::new()
        .route("/", get(root))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest(
            "/user",
            user_route().with_state(AppState { db: pool.clone() }),
        )
        .layer(Extension(pool));

    Ok(router.into())
}
