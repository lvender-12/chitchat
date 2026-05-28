use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::error::AppResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,
    email: String,
    exp: u64,
}

pub fn generate_jwt(uuid: String, email: String, secret: &str, exp: u64) -> AppResult<String> {
    let exp = Utc::now() + Duration::seconds(exp as i64);
    let claims = Claims {
        sub: uuid,
        email,
        exp: exp.timestamp() as u64,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str, secret: &str) -> AppResult<Claims> {
    let decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(decode.claims)
}
