use serde::{
    Serialize,
    Deserialize
};

use chrono::{
    Utc,
    Duration
};

use jsonwebtoken::{
    decode,
    encode,
    EncodingKey,
    Header,
    DecodingKey,
    Validation,
};

use crate::{
    utils::config,
    utils::errors::AppError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(
    user_id: &str,
) -> Result<String, AppError> {
    let secret = config::get_jwt_secret();

    let valid_for =  config::get_token_expiration_time()
        .parse::<i64>()
        .expect("Expiration time at .env is not a number.");

    let expiration = Utc::now() + Duration::minutes(valid_for);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp() as usize,
    };

    Ok(
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref())
        )?
    )
}

pub fn validate_token(
    token: &str,
) -> Result<Claims, AppError> {
    let secret = config::get_jwt_secret();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
