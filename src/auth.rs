use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
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
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{
    error::{AppError, UserError},
    AppState,
};

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

fn generate_token(key: &HS256Key) -> Result<String, AppError> {
    let claim = Claims::create(Duration::from_days(3));
    Ok(key.authenticate(claim)?)
}

fn is_valid_token(key: &HS256Key, token: &str) -> bool {
    let res = key.verify_token::<NoCustomClaims>(token, None);
    res.is_ok()
}

async fn does_user_exist(db: &Pool<Sqlite>, name: &str, password: &str) -> Result<bool, AppError> {
    let user = sqlx::query("select * from user where name = ? and password = ?")
        .bind(name)
        .bind(password)
        .fetch_optional(db)
        .await?;

    Ok(user.is_some())
}

pub async fn login(
    state: AppState,
    ip: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    login: Json<Login>,
) -> Result<impl IntoResponse, AppError> {
    let forwarded_for = headers
        .get("X-Forwarded-For")
        .map_or("not set", |f| f.to_str().unwrap_or("error"));

    if does_user_exist(&state.db, &login.name, &login.password).await? {
        let token = generate_token(&state.jwt_secret)?;
        println!(
            "new login: username: {} ip: {} X-Forwarded-For: {}",
            login.name,
            ip.ip(),
            forwarded_for
        );

        let cookie = Cookie::new("auth", token).to_string();
        Ok(([(SET_COOKIE, cookie)], "logged in").into_response())
    } else {
        println!(
            "failed login attempt: username: {} ip: {} X-Forwarded-For: {}",
            login.name,
            ip.ip(),
            forwarded_for
        );
        Err(UserError::LoginError.into())
    }
}

pub async fn logout() -> impl IntoResponse {
    let cookie = Cookie::new("auth", "").to_string();
    ([(SET_COOKIE, cookie)], "logged out").into_response()
}

pub async fn login_status() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn require_auth_middleware(
    cookies: CookieJar,
    state: AppState,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let is_logged_in = cookies
        .get("auth")
        .map(|c| is_valid_token(&state.jwt_secret, c.value()))
        .unwrap_or(false);

    if is_logged_in {
        Ok(next.run(request).await.into_response())
    } else {
        Err(UserError::AuthError.into())
    }
}
