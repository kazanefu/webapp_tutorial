use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
mod util;
use util::password_hash_check::{hash_password, verify_password};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Deserialize)]
struct SignupReq {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SignupRes {
    uid: String,
}

#[derive(Deserialize)]
struct LoginReq {
    uid: String,
    password: String,
}

#[derive(Serialize)]
struct LoginRes {
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("cwd: {:?}", std::env::current_dir()?);
    use sqlx::sqlite::SqlitePoolOptions;

    let db = SqlitePoolOptions::new()
        .connect("sqlite:users.db?mode=rwc")
        .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users(
            uid TEXT PRIMARY KEY,
            username TEXT,
            password_hash TEXT
        )
        ",
    )
    .execute(&db)
    .await?;

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(AppState { db })
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("backend running");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupReq>,
) -> Result<Json<SignupRes>, (StatusCode, String)> {
    let uid = Uuid::new_v4().to_string();
    if !is_valid_password(&req.password) || !is_valid_username(&req.username) {
        return Err((StatusCode::BAD_REQUEST, "ASCII only".to_string()));
    }

    let password_hash = tokio::task::spawn_blocking(move || hash_password(&req.password));

    sqlx::query("INSERT INTO users(uid,username,password_hash) VALUES(?,?,?)")
        .bind(&uid)
        .bind(&req.username)
        .bind(&password_hash.await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password hash error".to_string(),
            )
        })?)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    Ok(Json(SignupRes { uid }))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginRes>, (StatusCode, String)> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT username,password_hash FROM users WHERE uid=?",
    )
    .bind(&req.uid)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    })?;
    match row {
        Some((username, password_hash)) => {
            if verify_password(&password_hash, &req.password) {
                Ok(Json(LoginRes { username }))
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid UID or password".to_string(),
                ))
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid UID or password".to_string(),
        )),
    }
}

fn is_valid_password(password: &str) -> bool {
    password.is_ascii() && !password.is_empty()
}

fn is_valid_username(username: &str) -> bool {
    username.is_ascii() && !username.is_empty() && username.len() <= 32
}
