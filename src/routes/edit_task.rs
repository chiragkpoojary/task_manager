use actix_web::{web, HttpResponse};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use serde::Deserialize;

#[derive(Deserialize)]
struct PathInfo {
    id: String,
}

#[derive(Debug, Deserialize)]
struct Task {  
    task: String,
}

#[actix_web::put("/edit/{id}")]
pub async fn edit_task_(
    path: web::Path<PathInfo>,
    mongo_client: web::Data<Client>,
    updated_task: web::Json<Task>,
) -> HttpResponse {
   
                let collection: Collection<Task> = mongo_client.database("task").collection("tasks");


    // Convert the string ID to an ObjectId
    let obj_id = match ObjectId::parse_str(&path.id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid ID format"),
    };

    // Update the task
    let filter = doc! { "_id": obj_id };
    let update = doc! { "$set": { "task": &updated_task.task } };

   
    match collection.update_one(filter, update).await {
        Ok(result) => {
            if result.matched_count == 1 {
                HttpResponse::Ok().json("Task edited successfully")
            } else {
                HttpResponse::NotFound().json("Task not found in the list bro")
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Failed to edit task: {}", err))
        }
    }
}
