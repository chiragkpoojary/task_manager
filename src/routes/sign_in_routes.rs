 use actix_web::{web, HttpResponse, Responder, post};
 use bcrypt::verify;
 use mongodb::{Client, Collection};
 use mongodb::bson::doc;
 use serde::{Deserialize,Serialize};
use jwt_simple::prelude::*;

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
         Ok(user) => user,
         Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to find user: {}", e)),
     };

     // Verify user existence and password
     if let Some(user) = user {
         match verify(&data.password, &user.password_hash) {
             Ok(true) => {
                let key = HS256Key::generate();

            // Create claims valid for 2 hours
            let duration = Duration::from_secs(2 * 60 * 60); // 2 hours

            let jwt_duration = jwt_simple::prelude::Duration::from_secs(duration.as_secs() as u64);

            // Create claims with the specified duration
            let claims = Claims::create(jwt_duration)
                .with_issuer("your_app_name")
                .with_subject("user@example.com");

            // Generate token
            let token = key.authenticate(claims).unwrap();

            // Return the token in the response
            return HttpResponse::Ok().json(AuthResponse { token });            
             }
             Ok(false) => HttpResponse::Unauthorized().json("Invalid  password"),
             Err(e) => HttpResponse::InternalServerError().json(format!("Password verification failed: {}", e)),
         }
     } else {
         HttpResponse::Unauthorized().json("Invalid email or password")
     }
 }
