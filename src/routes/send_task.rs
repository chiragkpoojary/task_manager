use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use actix_session::Session;
use futures_util::stream::StreamExt;

#[derive(Debug, Deserialize,Serialize)]
struct Task {
  
    user_email: String,  
    task: String,
}

#[get("/tasks")]
pub async fn get_tasks(mongo_client: web::Data<Client>, session: Session) -> impl Responder {
    // Retrieve user email from the session
    if let Some(user_email) = session.get::<String>("user_email").unwrap() {
        let collection: Collection<Task> = mongo_client.database("auth").collection("tasks");
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
        HttpResponse::Unauthorized().json("User not logged in")
    }
}
