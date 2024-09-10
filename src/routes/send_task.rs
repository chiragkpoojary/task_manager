use actix_web::{get, web, HttpResponse, Responder,HttpRequest};
use jwt_simple::prelude::*;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

use futures_util::stream::StreamExt;

#[derive(Debug, Deserialize,Serialize)]
struct Task {
  
    user_email: String,  
    task: String,
}

#[get("/tasks")]
pub async fn get_tasks(
    req: HttpRequest,
    mongo_client: web::Data<Client>,
    key: web::Data<HS256Key>,
) -> impl Responder {

    // Extract and verify the access token
    let token = req.headers().get("Authorization").and_then(|header| {
        header.to_str().ok().map(|s| s.trim_start_matches("Bearer "))
    });

    if let Some(token) = token {
        if let Ok(claims) = key.verify_token::<NoCustomClaims>(token, None) {
            // Extract the user's email from the token claims
            let user_email = claims.subject.unwrap_or_default();

            // Access the MongoDB collection
            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");
            let filter = doc! { "user_email": &user_email };

            // Find tasks for the logged-in user
            let mut cursor = match collection.find(filter).await {
                Ok(cursor) => cursor,
                Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to fetch tasks: {}", e)),
            };

            let mut tasks = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(task) => tasks.push(task),
                    Err(e) => return HttpResponse::InternalServerError().json(format!("Failed to fetch task: {}", e)),
                }
            }

            HttpResponse::Ok().json(tasks)
        } else {
            HttpResponse::Unauthorized().json("Invalid token")
        }
    } else {
        HttpResponse::Unauthorized().json("Authorization token is missing")
    }
}
