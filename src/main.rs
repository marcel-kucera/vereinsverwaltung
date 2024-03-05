use std::str::FromStr;

use auth::{login, login_status, logout, require_auth_middleware};
use axum::{
    extract::{Host, Request, State},
    http::{HeaderValue, Method, StatusCode, Uri},
    middleware::{from_fn, from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

use include_dir::{include_dir, Dir};
use jwt_simple::algorithms::HS256Key;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

mod auth;
mod file;
mod member;
mod payment;

#[derive(Clone)]
pub struct AppStateStruct {
    db: sqlx::SqlitePool,
    jwt_secret: HS256Key,
}

pub type AppState = State<AppStateStruct>;

static FRONTEND: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/build");

#[tokio::main]
async fn main() {
    let is_old_database = Sqlite::database_exists("db").await.unwrap();
    if !is_old_database {
        println!("creating new database!");
        Sqlite::create_database("db").await.unwrap();
    }

    let db = SqlitePool::connect("sqlite:db").await.unwrap();

    if !is_old_database {
        let migration = include_str!("../db.sql");
        sqlx::query(migration).execute(&db).await.unwrap();
    }

    let state = AppStateStruct {
        db,
        jwt_secret: HS256Key::from_bytes("***REMOVED***".as_bytes()),
    };

    let api_auth_router = Router::new()
        .route(
            "/member",
            get(member::get_member)
                .post(member::post_member)
                .delete(member::delete_member)
                .put(member::put_member),
        )
        .route(
            "/payment",
            get(payment::get_payment)
                .post(payment::post_payment)
                .delete(payment::delete_payment),
        )
        .route(
            "/file",
            get(file::get_file)
                .post(file::post_file)
                .delete(file::delete_file),
        )
        .route("/filelist", get(file::get_filelist))
        .route("/status", get(login_status))
        .route_layer(from_fn_with_state(state.clone(), require_auth_middleware))
        .with_state(state.clone());

    let api_router = Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .merge(api_auth_router)
        .with_state(state);

    let app_router = Router::new()
        .nest("/api", api_router)
        .route("/", get(frontend_router))
        .route("/*rest", get(frontend_router))
        .layer(from_fn(cors_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_router).await.unwrap()
}

async fn cors_middleware(request: Request, next: Next) -> Response {
    let host = "http://localhost:5173";
    let mut res = match request.method() {
        &Method::OPTIONS => StatusCode::OK.into_response(),
        _ => next.run(request).await.into_response(),
    };
    res.headers_mut().insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str(host).unwrap(),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_str("GET,POST,UPDATE,DELETE,PUT").unwrap(),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_str("Content-Type").unwrap(),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_str("true").unwrap(),
    );
    res
}

async fn frontend_router(path: Uri) -> impl IntoResponse {
    let path = path.path().trim_start_matches('/');

    let filepath = if path.is_empty() { "index.html" } else { path };

    let mimetype_guess = mime_guess::from_path(filepath);
    let typestring = mimetype_guess
        .first_or_text_plain()
        .essence_str()
        .to_owned();

    let file = FRONTEND.get_file(filepath);
    match file {
        Some(data) => ([("content-type", typestring)], data.contents()).into_response(),
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}
