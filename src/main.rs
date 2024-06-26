use std::{env, net::SocketAddr};

use auth::{login, login_status, logout, require_auth_middleware};
use axum::{
    extract::{DefaultBodyLimit, Request, State},
    http::{HeaderValue, Method, StatusCode, Uri},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

use dotenv::dotenv;
use include_dir::{include_dir, Dir};
use jwt_simple::algorithms::HS256Key;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::signal::{
    ctrl_c,
    unix::{signal, SignalKind},
};

mod auth;
mod error;
mod file;
mod member;
mod payment;

#[derive(Clone)]
pub struct AppStateStruct {
    db: sqlx::SqlitePool,
    jwt_secret: HS256Key,
    host: String,
    inital_user_name: Option<String>,
    inital_user_password: Option<String>,
}

pub type AppState = State<AppStateStruct>;

static FRONTEND: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/build");

async fn create_config() -> AppStateStruct {
    dotenv().ok();

    println!(
        "starting vereinsverwaltung v{}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN")
    );

    // Create Database if it doesnt exist already
    let is_old_database = Sqlite::database_exists("db").await.unwrap();
    if !is_old_database {
        println!("no database found. creating new database");
        Sqlite::create_database("db").await.unwrap();
    }

    // Create db pool
    let db = SqlitePool::connect("sqlite:db").await.unwrap();

    // run migrations
    sqlx::migrate!().run(&db).await.unwrap();

    let state = AppStateStruct {
        db,
        jwt_secret: HS256Key::from_bytes(
            env::var("VEREINSVERWALTUNG_JWT_SECRET")
                .expect("setting VEREINSVERWALTUNG_JWT_SECRET is required")
                .as_bytes(),
        ),
        host: env::var("VEREINSVERWALTUNG_HOST").unwrap_or("localhost:3000".to_owned()),
        inital_user_name: env::var("VEREINSVERWALTUNG_USER").ok(),
        inital_user_password: env::var("VEREINSVERWALTUNG_PASSWORD").ok(),
    };

    // insert user if database has been created
    if !is_old_database {
        if let (Some(name), Some(password)) = (&state.inital_user_name, &state.inital_user_password)
        {
            println!("creating inital user {}", name);
            sqlx::query("INSERT INTO user (name,password) VALUES (?,?)")
                .bind(name)
                .bind(password)
                .execute(&state.db)
                .await
                .unwrap();
        } else {
            println!("no initial user created");
        }
    }

    println!("host: {}", &state.host);
    state
}

#[tokio::main]
async fn main() {
    let state = create_config().await;

    // protected api routes
    let api_auth_router = Router::new()
        .route(
            "/member",
            get(member::get_member)
                .post(member::post_member)
                .delete(member::delete_member)
                .put(member::put_member),
        )
        .route("/member/restore", post(member::restore_member))
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

    // api router with auth methods
    let api_router = Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .merge(api_auth_router)
        .with_state(state.clone());

    // final router with frontend
    let app_router = Router::new()
        .nest("/api", api_router)
        .route("/", get(frontend_router))
        .route("/*rest", get(frontend_router))
        .layer(from_fn_with_state(state.clone(), cors_middleware))
        .layer(DefaultBodyLimit::disable());

    // setup server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app_router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_handler())
    .await
    .unwrap();

    println!("shutting down!");
    state.db.close().await;
    println!("exiting!");
}

async fn shutdown_handler() {
    let ctrlc = ctrl_c();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    tokio::select! {
        _ = ctrlc => {}
        _ = sigterm.recv() => {}
        _ = sigint.recv() => {}
    }
}

async fn cors_middleware(state: AppState, request: Request, next: Next) -> Response {
    let host = &state.host;
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
