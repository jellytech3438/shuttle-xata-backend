use crate::*;

pub async fn user_create(
    State(appstate): State<AppState>,
    Json(data): Json<FormUser>,
) -> impl IntoResponse {
    let query_result = data.insert_into_db(&appstate.db).await;

    (axum::http::StatusCode::OK, Json(query_result)).into_response()
}

pub async fn users(State(appstate): State<AppState>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * FROM `user`")
        .fetch_all(&appstate.db)
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
                "id": row.try_get::<u32,_>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
                "phone_number": row.try_get::<String, _>("phone-number").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

pub async fn user_id(State(appstate): State<AppState>, Path(id): Path<u32>) -> impl IntoResponse {
    let search = format!("SELECT * FROM `user` where `user`.id = {}", id);
    let rows = match sqlx::query(&search).fetch_all(&appstate.db).await {
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
                "id": row.try_get::<u32,_>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
                "phone_number": row.try_get::<String, _>("phone-number").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}
