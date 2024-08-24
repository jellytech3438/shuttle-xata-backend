pub mod account;
pub mod card;
pub mod user;

use account::*;
use card::*;
use user::*;

use axum::{
    body::Body,
    extract::{FromRef, Json as FJson, Path, Query, Request, State},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::{
    extract::cookie::{Cookie, Key, SameSite},
    extract::PrivateCookieJar,
};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_runtime::SecretStore;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool, Row,
};
use std::{sync::Arc, time::Duration};
use time::{Duration as TimeDuration, OffsetDateTime};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Post {
    time: DateTime<Local>,
    categories: Vec<String>,
    title: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    db: PgPool,
    key: Key,
}

type ShareState = Arc<Mutex<AppState>>;

// this impl tells `SignedCookieJar` and `PrivateCookieJar how to access the
// key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
    paths(openapi, user_create, user_delete, user_update),
    components(schemas(FormUser))
)]
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
    let workspace_id = secrets.get("XATA_DB_WORKSPACE_ID").unwrap();
    let api_key = secrets.get("XATA_DB_API_KEY").unwrap();
    let region = secrets.get("XATA_DB_REGION").unwrap();
    let db_name = secrets.get("XATA_DB_NAME").unwrap();
    let db_branch = secrets.get("XATA_DB_BRANCH").unwrap();
    let db_url = format!(
        "postgresql://{workspace_id}:{api_key}@{region}.sql.xata.sh:5432/{db_name}:{db_branch}?sslmode=require"
    );
    println!("{}", db_url);
    // let mut ops: PgConnectOptions = db_url.parse().expect("parse error");
    // let mut pool = PgPool::connect_with(ops)
    //     .await
    //     .expect("Failed connect to database");
    let mut pool = PgPoolOptions::new()
        .max_connections(8)
        .idle_timeout(Duration::from_secs(120))
        .after_release(|conn, meta| {
            Box::pin(async move {
                // Only check connections older than 6 hours.
                if meta.age.as_secs() < 6 * 60 * 60 {
                    return Ok(true);
                }

                let total_memory_usage: i64 =
                    sqlx::query_scalar("select sum(used_bytes) from pg_backend_memory_contexts")
                        .fetch_one(conn)
                        .await?;

                // Close the connection if the backend memory usage exceeds 256 MiB.
                Ok(total_memory_usage <= (2 << 28))
            })
        })
        .connect(&db_url)
        .await
        .expect("Failed connect to database");
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);
    let appstate = Arc::new(Mutex::new(AppState {
        db: pool.clone(),
        key: Key::generate(),
    }));
    let router = Router::new()
        .route("/", get(root))
        .nest("/user", user_route().layer(Extension(appstate.clone())))
        .nest(
            "/account",
            account_route().with_state(appstate.clone().lock().await.to_owned()),
        )
        .nest("/card", card_route().layer(Extension(appstate.clone())))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(Extension(pool));

    Ok(router.into())
}
