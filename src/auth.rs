use axum::{
    extract::Request,
    http::{header::SET_COOKIE, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::{Claims, NoCustomClaims},
    reexports::coarsetime::Duration,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Error, Pool, Sqlite};

use crate::AppState;

#[derive(Deserialize, Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    name: String,
    password: String,
}

fn generate_token(key: &HS256Key) -> String {
    let claim = Claims::create(Duration::from_days(3));
    let token = key.authenticate(claim);
    token.unwrap()
}

fn is_valid_token(key: &HS256Key, token: &str) -> bool {
    let res = key.verify_token::<NoCustomClaims>(token, None);
    res.is_ok()
}

async fn does_user_exist(db: &Pool<Sqlite>, name: &str, password: &str) -> Result<bool, Error> {
    let user = sqlx::query("select * from user where name = ? and password = ?")
        .bind(name)
        .bind(password)
        .fetch_optional(db)
        .await?;

    Ok(user.is_some())
}

pub async fn login(state: AppState, login: Json<Login>) -> impl IntoResponse {
    if does_user_exist(&state.db, &login.name, &login.password)
        .await
        .unwrap()
    {
        let token = generate_token(&state.jwt_secret);

        let cookie = Cookie::new("auth", token).to_string();
        ([(SET_COOKIE, cookie)], "logged in").into_response()
    } else {
        (StatusCode::NOT_FOUND, "username or password is wrong").into_response()
    }
}

pub async fn logout() -> impl IntoResponse {
    let cookie = Cookie::new("auth", "").to_string();
    ([(SET_COOKIE, cookie)], "logged out".into_response()).into_response()
}

pub async fn login_status() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn require_auth_middleware(
    cookies: CookieJar,
    state: AppState,
    request: Request,
    next: Next,
) -> Response {
    match cookies
        .get("auth")
        .map(|c| is_valid_token(&state.jwt_secret, c.value()))
    {
        Some(true) => next.run(request).await.into_response(),
        _ => (StatusCode::UNAUTHORIZED, "you need to authorize").into_response(),
    }
}
