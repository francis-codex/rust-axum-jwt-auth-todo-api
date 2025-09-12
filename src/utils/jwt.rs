use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);
        
        Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_token(user_id: Uuid, jwt_secret: &str, expiration_hours: i64) -> Result<String, AppError> {
    let claims = Claims::new(user_id, expiration_hours);
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    
    encode(&header, &claims, &encoding_key).map_err(AppError::from)
}

pub fn verify_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();
    
    decode::<Claims>(token, &decoding_key, &validation)
        .map(|token_data| token_data.claims)
        .map_err(AppError::from)
}

pub fn extract_user_id_from_token(token: &str, jwt_secret: &str) -> Result<Uuid, AppError> {
    let claims = verify_token(token, jwt_secret)?;
    
    Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized)
}