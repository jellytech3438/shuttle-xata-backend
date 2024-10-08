use crate::*;

#[utoipa::path(
    post,
    path = "/user/create",
    request_body(
        content = FormUser,
        content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn user_create(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Json(data): Json<FormUser>,
) -> impl IntoResponse {
    let query_result = data.insert_into_db(&appstate.lock().await.db).await;

    (axum::http::StatusCode::OK, Json(query_result)).into_response()
}

pub async fn users(Extension(appstate): Extension<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * FROM \"user\"")
        .fetch_all(&appstate.lock().await.db)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<String,_>("xata_id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "mail": row.try_get::<String, _>("mail").unwrap_or_default(),
                "password": row.try_get::<String, _>("password").unwrap_or_default(),
                "create_date": row.try_get::<DateTime<Utc>,_>("xata_createdat").unwrap_or_default(),
                "update_date": row.try_get::<DateTime<Utc>,_>("xata_updatedat").unwrap_or_default()
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

// find user by id
pub async fn user_id(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"user\" where \"user\".xata_id = \'{}\'", id);
    let row = match sqlx::query(&search)
        .fetch_all(&appstate.lock().await.db)
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let user: FormUser = match &row.get(0) {
        &Some(sqlrow) => FormUser {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            name: sqlrow.try_get::<String, _>("name").unwrap_or_default(),
            mail: sqlrow.try_get::<String, _>("mail").unwrap_or_default(),
            password: sqlrow.try_get::<String, _>("password").unwrap_or_default(),
        },
        &None => FormUser::test_data(),
    };

    println!("{:?}", Json(&user));

    (axum::http::StatusCode::OK, Json(user)).into_response()
}

#[utoipa::path(
    put,
    path = "/user/:id",
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn user_update(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
    Json(data): Json<FormUser>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"user\" where \"user\".xata_id = \'{}\'", id);
    let row = match sqlx::query(&search)
        .fetch_all(&appstate.lock().await.db)
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let user: FormUser = match &row.get(0) {
        &Some(sqlrow) => FormUser {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            name: sqlrow.try_get::<String, _>("name").unwrap_or_default(),
            mail: sqlrow.try_get::<String, _>("mail").unwrap_or_default(),
            password: sqlrow.try_get::<String, _>("password").unwrap_or_default(),
        },
        &None => FormUser::test_data(),
    };

    user.update(&appstate.lock().await.db, data).await;
    // return old data as response
    (axum::http::StatusCode::OK, Json(user)).into_response()
}

#[utoipa::path(
    delete,
    path = "/user/:id",
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn user_delete(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"user\" where \"user\".xata_id = \'{}\'", id);
    let row = match sqlx::query(&search)
        .fetch_all(&appstate.lock().await.db)
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let user: FormUser = match &row.get(0) {
        &Some(sqlrow) => FormUser {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            name: sqlrow.try_get::<String, _>("name").unwrap_or_default(),
            mail: sqlrow.try_get::<String, _>("mail").unwrap_or_default(),
            password: sqlrow.try_get::<String, _>("password").unwrap_or_default(),
        },
        &None => FormUser::test_data(),
    };

    user.delete_from_db(&appstate.lock().await.db).await;

    (axum::http::StatusCode::OK, Json(user)).into_response()
}
