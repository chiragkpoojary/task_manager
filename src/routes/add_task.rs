use actix_web::{web, HttpResponse, Responder, post};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    // user_email: String,  
    task: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskRequest {
    pub task: String,
}

#[post("/addtask")]
pub async fn addtask(
    data: web::Json<TaskRequest>,
    mongo_client: web::Data<Client>,
    // key: web::Data<HS256Key>,
) -> impl Responder {

    // Extract and verify the access token
    // let token = req.headers().get("Authorization").and_then(|header| {
    //     header.to_str().ok().map(|s| s.trim_start_matches("Bearer "))
    // });

    // if let Some(token) = token {
    //     if let Ok(claims) = key.verify_token::<NoCustomClaims>(token, None) {
            // Extract the user's email from the token claims
            // let user_email = claims.subject.unwrap_or_default();

            // Access the MongoDB collection
            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");

            // Create a new task with the user's email
            let new_task = Task {
                // user_email,
                task: data.task.clone(),
            };

            // Insert the task into the database
            match collection.insert_one(new_task).await {
                Ok(_) => HttpResponse::Ok().json("Task added successfully"),
                Err(e) => HttpResponse::InternalServerError().json(format!("Failed to add task: {}", e)),
            }
    //     }
    //  else {
    //         HttpResponse::Unauthorized().json("Invalid token")
    //     }
    // } else {
    //     HttpResponse::Unauthorized().json("Authorization token is missing")
    // }
}
