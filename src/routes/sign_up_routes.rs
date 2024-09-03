use actix_web::{web, HttpResponse, Responder, post};
use bcrypt::{hash, DEFAULT_COST};
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use validator::Validate;
 
#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct UserSignUp {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
}

#[post("/signup")]
pub async fn sign_up(
    data: web::Json<UserSignUp>,
    mongo_client: web::Data<Client>
) -> impl Responder {
    // Validate input data
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let collection: Collection<User> = mongo_client.database("auth").collection("users");

    // Check if user already exists
    if collection.find_one(doc! { "email": &data.email }).await.expect("Failed to query user").is_some() {
        return HttpResponse::Conflict().json("User already exists");
    }

    // Hash the password
    let hashed_password = match hash(&data.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Password hashing failed: {}", e)),
    };

    // Create user with hashed password
    let user = User {
        email: data.email.clone(),
        password_hash: hashed_password,
    };

    // Insert the new user
    match collection.insert_one(user).await {
        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to add user: {}", e)),
    }
}
