use crate::routes::jwt_middleware::validator;
use actix_web::{post, web, Error, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct Task {
    user_email: String,
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
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {

    match validator(&auth).await {
        Ok(claims) => {

            let user_email = claims.sub;

            let task = Task {
                user_email: user_email.clone(),
                task: data.task.clone(),
            };

            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");

            collection.insert_one(task).await;

            Ok(HttpResponse::Ok().json("Task added successfully"))
        }
        Err(_) => {

            println!("JWT validation failed");
            Ok(HttpResponse::Unauthorized().json("Invalid token"))
        }
    }
}
