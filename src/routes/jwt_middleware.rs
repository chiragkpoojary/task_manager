use actix_web::Error;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::env;

use actix_web_httpauth::extractors::bearer::BearerAuth;


#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub _exp: usize,  
}


pub async fn validator(auth: &BearerAuth) -> Result<Claims, Error> {
    let token = auth.token();

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}
