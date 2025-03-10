use std::env;

fn get_from_env(key: &str) -> String {
    env::var(key).expect(&format!("{key} must be set"))
}

pub fn get_jwt_secret() -> String {
    get_from_env("JWT_SECRET")
}

pub fn get_token_expiration_time() -> String {
    get_from_env("TOKEN_EXPIRATION_TIME")
}