use crate::*;

pub fn user_route() -> Router {
    Router::new()
        .route("/", get(users))
        .route("/create", post(user_create))
        .route("/:id", get(user_id))
        .route("/:id", put(user_update))
        .route("/:id", delete(user_delete))
}
