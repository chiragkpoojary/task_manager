use actix_web::cookie::Cookie;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::verify;
use jwt_simple::prelude::*;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[post("/signin")]
pub async fn sign_in(
    data: web::Json<SignInRequest>,
    mongo_client: web::Data<Client>,
) -> impl Responder {
    // Access the MongoDB collection
    let collection: Collection<User> = mongo_client.database("auth").collection("users");

    // Find the user by email
    let user = match collection.find_one(doc! { "email": &data.email }).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().json("Invalid email or password"),
        Err(e) => {
            return HttpResponse::InternalServerError().json(format!("Failed to find user: {}", e))
        }
    };

    // Verify user existence and password
    if let Ok(true) = verify(&data.password, &user.password_hash) {
        // Retrieve the secret key (can be stored in environment variables for security)
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let key = HS256Key::from_bytes(secret_key.as_bytes());

        // Create claims valid for 2 hours
        let claims = Claims::create(Duration::from_hours(2))
            .with_issuer("task_manager")
            .with_subject(user.email.clone()); // Use the user's email as the subject

        // Generate token
        let token = match key.authenticate(claims) {
            Ok(token) => token,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(format!("Token generation failed: {}", e))
            }
        };
        let token_cookie = Cookie::build("token", token.clone())
            .http_only(true)
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict)
            .finish();
        // Return the token in the response
        HttpResponse::Ok()
            .cookie(token_cookie)
            .json(AuthResponse { token })
    } else {
        HttpResponse::Unauthorized().json("Invalid email or password")
    }
}
