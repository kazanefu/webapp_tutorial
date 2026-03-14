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
