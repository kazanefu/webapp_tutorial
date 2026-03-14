use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
mod util;
use axum::http::HeaderMap;
use axum::http::{HeaderValue, header};
use axum::response::IntoResponse;
use util::check_login_status::{get_session_id, me};
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

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS sessions(
            session_id TEXT PRIMARY KEY,
            uid TEXT
        )
        ",
    )
    .execute(&db)
    .await?;

    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_credentials(true)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/me", axum::routing::get(me))
        .route("/logout", post(logout))
        .with_state(AppState { db })
        .layer(cors);

    println!("backend running on {}", addr);

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

    let password_hash = tokio::task::spawn_blocking(move || hash_password(&req.password))
        .await
        .map_err(|e| {
            eprintln!("Blocking task error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        })?
        .map_err(|e| {
            eprintln!("Password hash error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        })?;

    sqlx::query("INSERT INTO users(uid,username,password_hash) VALUES(?,?,?)")
        .bind(&uid)
        .bind(&req.username)
        .bind(&password_hash)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    Ok(Json(SignupRes { uid }))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
            if !verify_password(&password_hash, &req.password) {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid UID or password".to_string(),
                ));
            }
            let session_id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO sessions(session_id,uid) VALUES(?,?)")
                .bind(&session_id)
                .bind(&req.uid)
                .execute(&state.db)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;
            let cookie = format!("session_id={}; HttpOnly; Path=/; SameSite=Lax", session_id);
            let mut res = Json(LoginRes { username }).into_response();
            res.headers_mut()
                .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
            Ok(res)
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid UID or password".to_string(),
        )),
    }
}

async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session_id =
        get_session_id(&headers).ok_or((StatusCode::UNAUTHORIZED, "Not logged in".to_string()))?;

    sqlx::query("DELETE FROM sessions WHERE session_id=?")
        .bind(&session_id)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    let cookie = "session_id=deleted; Path=/; Max-Age=0; HttpOnly";

    let mut res = "ok".into_response();

    res.headers_mut()
        .insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());

    Ok(res)
}

fn is_valid_password(password: &str) -> bool {
    password.is_ascii() && !password.is_empty()
}

fn is_valid_username(username: &str) -> bool {
    username.is_ascii() && !username.is_empty() && username.len() <= 32
}
