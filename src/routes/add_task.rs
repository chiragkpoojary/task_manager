use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
struct Task {
  
    // user_email: String,  
    task: String,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct TaskRequest {
    pub task: String,
}

#[post("/addtask")]
pub async fn addtask(
    data: web::Json<TaskRequest>,
    mongo_client: web::Data<Client>,
    // session: Session,
) -> impl Responder {

    // if let Some(user_email) = session.get::<String>("user_email").unwrap() {
        let collection: Collection<Task> = mongo_client.database("task").collection("task");


        let new_task = Task {
            // user_email,
            task: data.task.clone(),
        };

       
        match collection.insert_one(new_task).await {
            Ok(_) => HttpResponse::Ok().json("Task added successfully"),
            Err(e) => HttpResponse::InternalServerError().json(format!("Failed to add task: {}", e)),
        }
    // } else {
    //     HttpResponse::Unauthorized().json("User not logged in")
    // }
}
