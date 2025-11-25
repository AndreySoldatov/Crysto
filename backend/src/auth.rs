use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info, instrument};

use crate::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new().route("/signup", get(signup))
}

#[derive(Deserialize)]
struct UserSignup {
    username: String,
    password: String,
}

#[instrument(skip_all)]
async fn signup(
    State(appstate): State<AppState>,
    Json(payload): Json<UserSignup>,
) -> Result<impl IntoResponse, StatusCode> {
    let lower_username = payload.username.to_lowercase();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let auth_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            error!(error = %e, "Password hashing error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string();

    let insert_result = sqlx::query(
        "INSERT INTO users (username, auth_hash, master_key_cipher, master_key_nonce, kdf_salt)
        VALUES ($1, $2, $3, $4, $5)
    ",
    )
    .bind(&lower_username)
    .bind(&auth_hash)
    .bind("todo")
    .bind("todo")
    .bind("todo")
    .execute(&appstate.dbpool)
    .await;

    match insert_result {
        Ok(_) => {
            info!(username = %lower_username, "User successfully created");
            Ok((
                StatusCode::CREATED,
                Json(json!({ "username": lower_username })),
            ))
        }
        Err(e) => {
            if let sqlx::Error::Database(db_error) = &e {
                if db_error.is_unique_violation() {
                    info!(username = %lower_username, "User already exists");
                    return Err(StatusCode::CONFLICT);
                }
            }

            error!(error = %e, "Error inserting user");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
