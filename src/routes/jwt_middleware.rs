use actix_web::{dev::ServiceRequest, Error};

use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::env;


use actix_web_httpauth::extractors::bearer::BearerAuth;

// Struct representing the JWT Claims
#[derive(Debug, Deserialize)]
struct Claims {
    sub: String, // Subject (user ID or email)
    exp: usize,  // Expiration timestamp
}

// Function to validate JWT
pub async fn validator(req: &ServiceRequest, auth: BearerAuth) -> Result<(), Error> {
    let token = auth.token();

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(_) => Ok(()),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}
