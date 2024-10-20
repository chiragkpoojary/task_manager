use crate::routes::jwt_middleware::validator;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::stream::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize}; // Struct representing a Task
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Keep MongoDB's default _id field
    task: String, // Store the task name separately
}

// GET /tasks endpoint to fetch tasks
#[get("/tasks")]
pub async fn get_tasks(mongo_client: web::Data<Client>, auth: BearerAuth) -> impl Responder {
    // Validate the JWT token
    match validator(&auth).await {
        Ok(claims) => {
            // Extract user email from claims
            let user_email = claims.sub;

            // Access the MongoDB collection
            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");

            // Create a filter to find tasks for the logged-in user
            let filter = doc! { "user_email": &user_email };

            // Find tasks for the logged-in user
            match collection.find(filter).await {
                Ok(mut cursor) => {
                    let mut tasks = Vec::new();
                    while let Some(result) = cursor.next().await {
                        match result {
                            Ok(task) => tasks.push(task),
                            Err(e) => {
                                return HttpResponse::InternalServerError()
                                    .json(format!("Failed to fetch task: {}", e))
                            }
                        }
                    }
                    HttpResponse::Ok().json(tasks)
                }
                Err(e) => {
                    eprintln!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Failed to fetch tasks: {}", e))
                }
            }
        }
        Err(_) => {
            // Token validation failed
            HttpResponse::Unauthorized().json("Invalid token")
        }
    }
}
