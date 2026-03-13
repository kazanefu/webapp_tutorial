use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

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
            password TEXT
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

async fn signup(State(state): State<AppState>, Json(req): Json<SignupReq>) -> Json<SignupRes> {
    let uid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users(uid,username,password) VALUES(?,?,?)")
        .bind(&uid)
        .bind(&req.username)
        .bind(&req.password)
        .execute(&state.db)
        .await
        .unwrap();

    Json(SignupRes { uid })
}

async fn login(State(state): State<AppState>, Json(req): Json<LoginReq>) -> Json<LoginRes> {
    let row =
        sqlx::query_as::<_, (String,)>("SELECT username FROM users WHERE uid=? AND password=?")
            .bind(&req.uid)
            .bind(&req.password)
            .fetch_one(&state.db)
            .await
            .unwrap();

    Json(LoginRes { username: row.0 })
}
