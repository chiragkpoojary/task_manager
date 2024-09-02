// use actix_web::{web, HttpResponse, Responder, post};
// use actix_session::Session;
// use bcrypt::verify;
// use mongodb::{Client, Collection};
// use mongodb::bson::doc;
// use serde::{Deserialize,Serialize};

// #[derive(Debug, Deserialize)]
// pub struct SignInRequest {
//     pub email: String,
//     pub password: String,
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub email: String,
//     pub password_hash: String,
// }
// #[post("/signin")]
// pub async fn sign_in(
//     data: web::Json<SignInRequest>,
//     mongo_client: web::Data<Client>,
//     session: Session,
// ) -> impl Responder {
//     // Access the MongoDB collection
//     let collection: Collection<User> = mongo_client.database("auth").collection("users");

//     // Find the user by email
//     let user = match collection.find_one(doc! { "email": &data.email }).await {
//         Ok(user) => user,
//         Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to find user: {}", e)),
//     };

//     // Verify user existence and password
//     if let Some(user) = user {
//         match verify(&data.password, &user.password_hash) {
//             Ok(true) => {
//                 // Set a session variable
//                 session.insert("user_email", &data.email).expect("Failed to set session");
//                 HttpResponse::Ok().json("Sign in successful")
//             }
//             Ok(false) => HttpResponse::Unauthorized().json("Invalid  password"),
//             Err(e) => HttpResponse::InternalServerError().json(format!("Password verification failed: {}", e)),
//         }
//     } else {
//         HttpResponse::Unauthorized().json("Invalid email or password")
//     }
// }
