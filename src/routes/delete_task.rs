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

#[actix_web::delete("/delete/{id}")]
pub async fn delete_task_(
    path: web::Path<PathInfo>,
    mongo_client: web::Data<Client>,
) -> HttpResponse {
  
            let collection: Collection<Task> = mongo_client.database("task").collection("tasks");

    // Convert the string ID to an ObjectId
    let obj_id = match ObjectId::parse_str(&path.id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid ID format"),
    };


    let filter = doc! { "_id": obj_id };


    match collection.delete_one(filter).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                HttpResponse::Ok().json("Task deleted successfully")
            } else {
                HttpResponse::NotFound().json("Task not found in the list bro")
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Failed to delete task: {}", err))
        }
    }
}
