use crate::*;

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/", get(users))
        .route("/create", post(user_create))
        .route("/:id", get(user_id))
    // .route("/:id/update", get(user_id).put(user_update))
    // .route("/:id/delete", get(user_id).delete(user_delete))
}
