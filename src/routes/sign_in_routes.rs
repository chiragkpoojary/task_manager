use mongodb:: {Client, Collection};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
}



#[post("/")]
pub async fn sign_in_routes1(
    data: web::Json<User>,
    mongo_client: web::Data<Client>,
) -> impl Responder {
    let collection: Collection<User> = mongo_client
        .database("auth")
        .collection("users");

    let user = User {
        name: data.name.clone(),
        password: data.password.clone(), 
    };

    match collection.insert_one(user).await {
        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to add user: {}", e)),
    }
}