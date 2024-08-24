use crate::*;

pub fn card_route() -> Router {
    Router::new()
        .route("/", get(cards))
        .route("/create", post(card_create))
        .route("/:id", get(card_id))
        .route("/:id", put(card_update))
        .route("/:id", delete(card_delete))
}
