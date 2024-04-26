use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    UserError(#[from] UserError),

    #[error("JWT Error")]
    JWTError(#[from] jwt_simple::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("error during request: {}\n{:?}", self, self);
        match self {
            AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
            }
            AppError::UserError(e) => e.into_response(),
            AppError::JWTError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"JWT Token Error (Bitte mir sagen, falls du das siehst. Sollte eigentlich nie passieren)").into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Not found")]
    NotFoundError,
    #[error("Duplicate")]
    DuplicateError,
    #[error("Wrong username or password")]
    LoginError,
    #[error("Must provide filename")]
    FileNameMissingError,
    #[error("You must be authorized")]
    AuthError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let statuscode = match self {
            UserError::NotFoundError => StatusCode::NOT_FOUND,
            UserError::DuplicateError => StatusCode::BAD_REQUEST,
            UserError::LoginError => StatusCode::NOT_FOUND,
            UserError::FileNameMissingError => StatusCode::BAD_REQUEST,
            UserError::AuthError => StatusCode::UNAUTHORIZED,
        };

        (statuscode, self.to_string()).into_response()
    }
}
