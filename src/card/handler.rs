use crate::*;

#[utoipa::path(
    post,
    path = "/card/create",
    request_body(
        content = FormUser,
        content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn card_create(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Json(data): Json<FormCard>,
) -> impl IntoResponse {
    let query_result = data.insert_into_db(&appstate.lock().await.db).await;

    (axum::http::StatusCode::OK, Json(query_result)).into_response()
}

pub async fn cards(Extension(appstate): Extension<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * FROM \"card\"")
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

    let cards: Vec<serde_json::Value> = rows
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

    (axum::http::StatusCode::OK, Json(cards)).into_response()
}

// find card by id
pub async fn card_id(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"card\" where \"card\".xata_id = \'{}\'", id);
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

    let card: FormCard = match &row.get(0) {
        &Some(sqlrow) => FormCard {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            uid: sqlrow.try_get::<String, _>("user_id").unwrap_or_default(),
            title: sqlrow.try_get::<String, _>("title").unwrap_or_default(),
            description: sqlrow
                .try_get::<String, _>("description")
                .unwrap_or_default(),
            image: sqlrow.try_get::<String, _>("image").unwrap_or_default(),
        },
        &None => FormCard::test_data(),
    };

    println!("{:?}", Json(&card));

    (axum::http::StatusCode::OK, Json(card)).into_response()
}

#[utoipa::path(
    put,
    path = "/card/:id",
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn card_update(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
    Json(data): Json<FormCard>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"card\" where \"card\".xata_id = \'{}\'", id);
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

    let card: FormCard = match &row.get(0) {
        &Some(sqlrow) => FormCard {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            uid: sqlrow.try_get::<String, _>("user_id").unwrap_or_default(),
            title: sqlrow.try_get::<String, _>("title").unwrap_or_default(),
            description: sqlrow
                .try_get::<String, _>("description")
                .unwrap_or_default(),
            image: sqlrow.try_get::<String, _>("image").unwrap_or_default(),
        },
        &None => FormCard::test_data(),
    };

    card.update(&appstate.lock().await.db, data).await;
    // return old data as response
    (axum::http::StatusCode::OK, Json(card)).into_response()
}

#[utoipa::path(
    delete,
    path = "/card/:id",
    responses(
        (status = 200),
        (status = 400)
    ))]
pub async fn card_delete(
    Extension(appstate): Extension<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let search = format!("SELECT * FROM \"card\" where \"card\".xata_id = \'{}\'", id);
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

    let card: FormCard = match &row.get(0) {
        &Some(sqlrow) => FormCard {
            id: sqlrow.try_get::<String, _>("xata_id").unwrap_or_default(),
            uid: sqlrow.try_get::<String, _>("user_id").unwrap_or_default(),
            title: sqlrow.try_get::<String, _>("title").unwrap_or_default(),
            description: sqlrow
                .try_get::<String, _>("description")
                .unwrap_or_default(),
            image: sqlrow.try_get::<String, _>("image").unwrap_or_default(),
        },
        &None => FormCard::test_data(),
    };

    card.delete_from_db(&appstate.lock().await.db).await;

    (axum::http::StatusCode::OK, Json(card)).into_response()
}
