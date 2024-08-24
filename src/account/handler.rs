use crate::*;

// pub async fn account_register(
//     Extension(appstate): Extension<Arc<Mutex<AppState>>>,
//     Json(data): Json<LoginDetails>,
// ) -> impl IntoResponse {
//     let query_result = data.insert_into_db(&appstate.lock().await.db).await;
//
//     (axum::http::StatusCode::OK, Json(())).into_response()
// }

pub async fn validate_session(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> (PrivateCookieJar, Response) {
    let Some(cookie) = jar
        .get("session-key")
        .map(|cookie| cookie.value().to_owned())
    else {
        println!("Couldn't find a cookie in the jar");
        return (
            jar,
            (axum::http::StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        );
    };

    // attempt to find the created session
    let find_session = sqlx::query(
        format!(
            "SELECT * FROM \"sessions\" WHERE \"session\".session_id = {}",
            cookie
        )
        .as_ref(),
    )
    .execute(&state.db)
    .await;

    // if the created session is OK, carry on as normal and run the route - else, return 403
    match find_session {
        Ok(res) => (jar, next.run(request).await),
        Err(_) => (
            jar,
            (axum::http::StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        ),
    }
}

pub async fn account_login(
    State(mut state): State<AppState>,
    jar: PrivateCookieJar,
    Json(login): Json<LoginDetails>,
    // ) -> Result<(PrivateCookieJar, axum::http::StatusCode), axum::http::StatusCode> {
) -> impl IntoResponse {
    let row = match sqlx::query(
        format!(
            "SELECT * FROM \"user\" WHERE \"user\".mail = \'{}\'",
            &login.mail
        )
        .as_ref(),
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(rows) => rows,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    // "value"
    let session_id = rand::random::<u64>().to_string();

    let user: serde_json::Value = match row {
        Some(row) => {
            json!({
                "id": row.try_get::<String,_>("xata_id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "mail": row.try_get::<String, _>("mail").unwrap_or_default(),
                "password": row.try_get::<String, _>("password").unwrap_or_default(),
                "create_date": row.try_get::<DateTime<Utc>,_>("xata_createdat").unwrap_or_default(),
                "update_date": row.try_get::<DateTime<Utc>,_>("xata_updatedat").unwrap_or_default()
            })
        }
        None => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    sqlx::query(
        format!(
            "INSERT INTO \"session\" (session_id, user_id) VALUES (\'{}\', \'{}\')",
            &session_id,
            user.get("id").unwrap()
        )
        .as_ref(),
    )
    .execute(&state.db)
    .await
    .expect("Couldn't insert session :(");

    let cookie = Cookie::build(("session-key", session_id))
        // secure true: only website starts with 'https' can access it
        // .secure(true)
        // SameSite::Strict: super restrict
        // SameSite::Lax: less strict( only 'get' method will pass)
        .same_site(SameSite::Lax)
        .http_only(true)
        .expires(OffsetDateTime::now_utc() + TimeDuration::hours(1))
        .path("/")
        .finish();

    Ok((jar.add(cookie), axum::http::StatusCode::OK))
}

pub async fn account_logout(
    State(appstate): State<AppState>,
    jar: PrivateCookieJar,
) -> impl IntoResponse {
    let Some(cookie) = jar
        .get("session-key")
        .map(|cookie| cookie.value().to_owned())
    else {
        println!("not get cookie");
        return Ok(jar);
    };

    match sqlx::query(
        format!(
            "DELETE FROM \"session\" WHERE \"session\".session_id = \'{}\'",
            &cookie
        )
        .as_ref(),
    )
    .execute(&appstate.db)
    .await
    {
        Ok(_) => Ok(jar.remove(Cookie::build("session-key").path("/"))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn account_forgotpw(
    State(appstate): State<AppState>,
    Json(email_recipient): Json<String>,
) -> impl IntoResponse {
    // let new_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    //
    // let hashed_password = bcrypt::hash(&new_password, 10).unwrap();
    //
    // sqlx::query("UPDATE users SET password = $1 WHERE email = $2")
    //     .bind(hashed_password)
    //     .bind(email_recipient)
    //     .execute(&state.postgres)
    //     .await;
    //
    // let credentials = Credentials::new(state.smtp_email, state.smtp_password);
    //
    // let message = format!("Hello!\n\n Your new password is: {new_password} \n\n Don't share this with anyone else. \n\n Kind regards, \nZest");
    //
    // let email = Message::builder()
    //     .from("noreply <your-gmail-address-here>".parse().unwrap())
    //     .to(format!("<{email_recipient}>").parse().unwrap())
    //     .subject("Forgot Password")
    //     .header(ContentType::TEXT_PLAIN)
    //     .body(message)
    //     .unwrap();
    //
    // // build the SMTP relay with our credentials - in this case we'll be using gmail's SMTP because it's free
    // let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(credentials)
    //     .build();
    //
    // // this part x`doesn't really matter since we don't want the user to explicitly know if they've actually received an email or not for security purposes, but if we do then we can create an output based on what we return to the client
    // match mailer.send(&email) {
    //     Ok(_) => (StatusCode::OK, "Sent".to_string()).into_response(),
    //     Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {e}")).into_response(),
    // }
}
