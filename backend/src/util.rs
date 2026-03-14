pub mod password_hash_check {
    use argon2::{
        Argon2,
        password_hash::{
            Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };

    pub fn hash_password(password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    pub fn verify_password(hash: &str, password: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(hash) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }
}

pub mod check_login_status {
    use axum::http::HeaderMap;
    pub fn get_session_id(headers: &HeaderMap) -> Option<String> {
        headers
            .get(axum::http::header::COOKIE)?
            .to_str()
            .ok()?
            .split("; ")
            .find_map(|c| {
                let mut parts = c.split('=');
                match (parts.next(), parts.next()) {
                    (Some("session_id"), Some(v)) => Some(v.to_string()),
                    _ => None,
                }
            })
    }
    use crate::*;
    use axum::{Json, extract::State, http::StatusCode};
    pub async fn me(
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> Result<Json<LoginRes>, (StatusCode, String)> {
        let session_id = get_session_id(&headers)
            .ok_or((StatusCode::UNAUTHORIZED, "0: Not logged in".to_string()))?;

        let row = sqlx::query_as::<_, (String,)>(
            "
        SELECT username
        FROM users
        JOIN sessions ON users.uid = sessions.uid
        WHERE sessions.session_id = ?
        ",
        )
        .bind(session_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

        match row {
            Some((username,)) => Ok(Json(LoginRes { username })),
            None => Err((StatusCode::UNAUTHORIZED, "1: Not logged in".to_string())),
        }
    }
}
