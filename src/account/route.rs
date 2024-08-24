use crate::*;

pub fn account_route() -> Router<AppState> {
    Router::new()
        // .route("/register", post(account_register))
        // POST /login: keep the user information into session
        .route("/login", post(account_login))
        // GET /logout: clear session
        .route("/logout", get(account_logout))

    // .route("/forgetpw", post(accout_forgetpw))
}
