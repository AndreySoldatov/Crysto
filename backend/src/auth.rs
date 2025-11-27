use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::{FromRequestParts, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::{debug, error, info, instrument};

use crate::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/check-username", get(check_username))
        .route("/login", post(login))
        .route("/protected", get(protected))
}

#[derive(Deserialize)]
struct SignupRequestBody {
    username: String,
    password: String,

    master_key_cipher: String,
    master_key_nonce: String,
    kdf_salt: String,
}

#[instrument(skip_all)]
async fn signup(
    State(appstate): State<AppState>,
    Json(payload): Json<SignupRequestBody>,
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
    .bind(&payload.master_key_cipher)
    .bind(&payload.master_key_nonce)
    .bind(&payload.kdf_salt)
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

#[derive(Deserialize)]
struct CheckUsernameQuery {
    username: String,
}

#[instrument(skip_all)]
async fn check_username(
    State(appstate): State<AppState>,
    Query(request): Query<CheckUsernameQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let lower_username = request.username.to_lowercase();

    let username_exists = check_username_in_db(&lower_username, &appstate.dbpool)
        .await
        .map_err(|e| {
            error!(error = %e, "Error reading username from database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let username_exists = debug!(username = %lower_username, exists = %username_exists, "checked username availability");
    Ok((StatusCode::OK, Json(json!({ "exists": username_exists }))))
}

async fn check_username_in_db(username: &str, dbpool: &PgPool) -> Result<bool, sqlx::Error> {
    Ok(sqlx::query("SELECT 1 FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(dbpool)
        .await?
        .is_some())
}

#[derive(Deserialize)]
struct LoginRequestBody {
    username: String,
    password: String,
}

#[instrument(skip_all)]
async fn login(
    State(appstate): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequestBody>,
) -> Result<(CookieJar, impl IntoResponse), StatusCode> {
    let lower_username = request.username.to_lowercase();

    let (user_id, auth_hash): (i64, String) =
        sqlx::query_as("SELECT id, auth_hash FROM users WHERE username = $1")
            .bind(&lower_username)
            .fetch_optional(&appstate.dbpool)
            .await
            .map_err(|e| {
                error!(error = %e, "Error fetching auth_hash from the database");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or_else(|| {
                debug!(username = %lower_username, "User not found in the database");
                StatusCode::UNAUTHORIZED
            })?;

    let parsed_hash = PasswordHash::new(&auth_hash).map_err(|e| {
        error!(error = %e, hash = %auth_hash, "auth hash parsing error!");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        debug!("password hashes do not match");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt =
        generate_jwt(appstate.config.jwt_secret.as_bytes(), &user_id.to_string()).map_err(|e| {
            error!(error = %e, "Error generating JWT!");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let refresh_token: Vec<u8> = rand::random_iter().take(32).collect();
    let refresh_base64 = BASE64_STANDARD.encode(&refresh_token);
    let refresh_hash = Sha256::digest(&refresh_token);
    let refresh_hash_base64 = BASE64_STANDARD.encode(&refresh_hash);

    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash)
        VALUES ($1, $2)",
    )
    .bind(user_id)
    .bind(refresh_hash_base64)
    .execute(&appstate.dbpool)
    .await
    .map_err(|e| {
        error!(error = %e, "Error inserting new refresh token into database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    debug!(username = %lower_username, "User Authorized successfully");
    Ok((
        jar.add(
            Cookie::build(("refresh_token", refresh_base64))
                .http_only(true)
                .same_site(SameSite::Lax),
        ),
        Json(json!({ "access_token": jwt, "token_type": "Bearer", "expires_in": 15 * 60 })),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

fn generate_jwt(secret: &[u8], sub: &str) -> anyhow::Result<String> {
    let iat = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let exp = iat + 15 * 60; // Expiration = Issued_at + 15 minutes

    let claims = Claims {
        sub: sub.to_string(),
        iat,
        exp,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;

    Ok(token)
}

fn validate_jwt(secret: &[u8], token: &str) -> jsonwebtoken::errors::Result<Claims> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|t| t.claims)
}

struct AuthorizedUser(i64);

impl<S> FromRequestParts<S> for AuthorizedUser
where
    S: Send + Sync,
    State<AppState>: FromRequestParts<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let State(AppState { config, .. }) = State::from_request_parts(parts, state)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

        let jwt_str = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or((StatusCode::UNAUTHORIZED, "No authorization header"))?
            .to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid header format"))?
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid header format"))?;

        let claims =
            validate_jwt(config.jwt_secret.as_bytes(), jwt_str).map_err(|e| match e.kind() {
                ErrorKind::ExpiredSignature => (StatusCode::UNAUTHORIZED, "Token expired"),
                _ => (StatusCode::UNAUTHORIZED, "Token validation error"),
            })?;

        // Unwrap here because if JWT passed validation, then I'm sure that
        // my server issued it and the sub field is the appropriate numeric type
        Ok(AuthorizedUser(claims.sub.parse().unwrap()))
    }
}

async fn protected(
    AuthorizedUser(user_id): AuthorizedUser,
    State(appstate): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let (username,): (String,) = sqlx::query_as("SELECT username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&appstate.dbpool)
        .await
        .map_err(|e| {
            error!(error = %e, "Error fetching username from the database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(format!("Hello, {username}!"))
}
